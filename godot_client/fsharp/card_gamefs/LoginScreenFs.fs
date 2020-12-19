namespace CardGame

open Godot
open JsonData
open System.Threading.Tasks
open FSharp.Json
open FSharp.Json.Json
open FSharp.Control.Tasks
open FSharp.Control.Tasks.NonAffine

type LoginScreenFs() as this =
    inherit Control()

    let userNameNode =
        lazy (this.GetNode<LineEdit>(new NodePath("UserName")))

    let passwordNode =
        lazy (this.GetNode<LineEdit>(new NodePath("Password")))


    let mutable currentlyProcessing: Option<Poll<bool>> = None

    override this._Process(delta) =
        currentlyProcessing
        |> poll.TryPoll(fun x ->
            if x then
                this.GetTree().ChangeScene("res://src/character_select.tscn")
                |> ignore
            currentlyProcessing <- None)
        |> ignore

    member this._OnLoginButtonpressed() =
        if currentlyProcessing.IsNone then
            currentlyProcessing <-
                (PollingClient.connect "127.0.0.1" 3030 false)
                |> poll.AndThenOk(fun _ ->
                    (PollingClient.login userNameNode.Value.Text passwordNode.Value.Text)
                    |> poll.Map(fun x -> Ok(x)))
                |> poll.Map(fun x ->
                    match x with
                    | Ok (x) -> x
                    | _ -> false)
                |> Some
