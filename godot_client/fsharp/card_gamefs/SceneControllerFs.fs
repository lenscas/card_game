namespace CardGame

open Godot

type SceneControllerFs() =
    inherit Node()

    let mutable currentRequest: Option<Poll<unit>> = None

    override this._Process delta = currentRequest |> Poll.TryIgnorePoll


    member this._OnSelectedCharacter(id: int) =
        Globals.SetCurrentId id
        currentRequest <-
            id
            |> PollingClient.isCharacterInBattle
            |> Poll.AfterOk(fun x ->
                (if x then "res://src/Battle.tscn" else "res://src/Dungeon.tscn"
                 |> ResourceLoader.Load :?> PackedScene).Instance()
                |> this.AddChild)
            |> Poll.IgnoreResult
            |> Some
