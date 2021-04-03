namespace CardGame

open Godot

type DungeonFs() as this =
    inherit Node2D()

    let mutable dungeonRequest = None

    let dungeonTileNode =
        lazy (this.GetNode<DungeonTilesFs>(new NodePath("DungeonTiles")))

    member this.GetDungeon currentId =
        this.Show()

        dungeonRequest <-
            currentId
            |> PollingClient.getDungeon
            |> Poll.AfterOk
                (fun x ->
                    match x with
                    | Some (x) ->
                        dungeonTileNode.Value._GotDungeon (x)
                        this.Show()
                    | None -> ())
            |> Poll.IgnoreResult
            |> Some

    member this._UpdateDungeon id = this.GetDungeon id

    override this._Process _ = dungeonRequest |> Poll.TryIgnorePoll
