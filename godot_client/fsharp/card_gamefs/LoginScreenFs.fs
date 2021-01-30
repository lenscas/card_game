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


    let mutable currentlyProcessing: Option<Poll<unit>> = None

    override this._Process(delta) =
        currentlyProcessing |> Poll.TryIgnorePoll

    member this._OnLoginButtonpressed() =
        if currentlyProcessing.IsNone then
            currentlyProcessing <-
                Poll.poll {
                    let! res = PollingClient.connect "127.0.0.1" 3030 false

                    let! canLogin =
                        match res with
                        | Ok (_) -> PollingClient.login userNameNode.Value.Text passwordNode.Value.Text
                        | Result.Error (x) ->
                            GD.PrintErr(x)
                            Poll.Ready(false)

                    if canLogin then
                        this
                            .GetTree()
                            .ChangeScene("res://src/character_select.tscn")
                        |> ignore


                    currentlyProcessing <- None

                }
                |> Poll.IgnoreResult
                |> Some
