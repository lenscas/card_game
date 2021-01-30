namespace CardGame

open Godot

type DungeonFs() =
    inherit Node2D()

    let mutable dungeonRequest = None

    member this.GetDungeon currentId =
        this.Show()

        dungeonRequest <-
            currentId
            |> PollingClient.getDungeon
            |> Poll.AfterOk
                (fun x ->
                    match x with
                    | Some (x) ->
                        this.EmitSignal("GotDungeonLayout", (FSharp.Json.Json.serialize x))
                        this.Show()
                    | None -> ())
            |> Poll.IgnoreResult
            |> Some

    override this._Process _ = dungeonRequest |> Poll.TryIgnorePoll
