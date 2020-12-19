namespace CardGame

open Godot
open Microsoft.FSharp.Core.Operators

[<Signal>]
type MySignal = delegate of unit -> unit

type character_selectFs() as this =
    inherit Control()


    let charSelectButton =
        lazy (this.GetNode<Button>(new NodePath("Button")))

    let mutable options = [||]

    let getCharacterRequest =
        PollingClient.characeterSelect ()
        |> Poll.After(fun x ->
            match x with
            | Ok (x) ->
                match x with
                | Some (x) ->
                    if x.characters.Length = 0
                    then charSelectButton.Value.Text <- "New character"
                    else charSelectButton.Value.Text <- "Current character"
                    options <- x.characters
                    charSelectButton.Value.Show()
                | None -> ()
            | Result.Error (_) -> ())

    let mutable selectCharacter: Option<Poll<unit>> = None




    override this._Ready() = charSelectButton.Value.Hide()

    override this._Process(delta) =
        getCharacterRequest |> Poll.ignorePoll
        selectCharacter |> Poll.TryIgnorePoll

    member this.OnCharSelectButtonPressed() =
        selectCharacter <-
            match options |> Array.toList with
            | [] -> PollingClient.createCharacter ()
            | top :: _ -> Poll.Ready(Ok(Some(top)))
            |> Poll.After(fun x ->
                match x with
                | Ok (x) ->
                    match x with
                    | Some (x) -> this.EmitSignal("SelectedCharacter", x)
                    | None -> ()
                | Result.Error (_) -> ())
            |> Poll.Map ignore
            |> Some
