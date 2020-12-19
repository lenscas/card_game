namespace CardGame

open Godot

type DungeonFs() as this =
    inherit Node2D()

    let dungeonRequest =
        Globals.getCurrentId ()
        |> PollingClient.getDungeon
        |> poll.AfterOk(fun x ->
            match x with
            | Some (x) -> this.EmitSignal("GotDungeonLayout", (FSharp.Json.Json.serialize x))
            | None -> ())
        |> poll.IgnoreResult

    override this._Process delta = dungeonRequest.Poll() |> ignore
