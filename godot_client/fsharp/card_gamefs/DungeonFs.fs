namespace CardGame

open Godot

type DungeonFs() as this =
    inherit Node2D()

    let dungeonRequest =
        Globals.getCurrentId ()
        |> PollingClient.getDungeon
        |> Poll.AfterOk(fun x ->
            match x with
            | Some (x) -> this.EmitSignal("GotDungeonLayout", (FSharp.Json.Json.serialize x))
            | None -> ())
        |> Poll.IgnoreResult

    override this._Process delta = dungeonRequest.Poll() |> ignore
