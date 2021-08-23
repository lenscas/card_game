namespace CardGame

open Godot

type SceneControllerFs() as this =
    inherit Node2D()

    let mutable currentRequest : Option<Poll<unit>> = None

    let DungeonNode =
        lazy (this.GetNode<DungeonFs>(new NodePath("Dungeon")))

    let ArenaNode =
        lazy (this.GetNode<ArenaFs>(new NodePath("Arena")))


    override this._Ready() =
        this
            .GetNode(new NodePath("Dungeon/DungeonTiles"))
            .Connect("EnteredBattle", this, "OnEnteredBattle")
        |> ignore

    override __._Process delta = currentRequest |> Poll.TryIgnorePoll

    member __.OnEnteredBattle() =
        DungeonNode.Value.Hide()

        Globals.getCurrentId ()
        |> ArenaNode.Value.GetArena

    member __.OnBattleEnded() =
        ArenaNode.Value.Hide()

        Globals.getCurrentId ()
        |> DungeonNode.Value.GetDungeon

    member __._OnSelectedCharacter(id: int) =
        GD.Print("in on select")
        Globals.SetCurrentId id

        currentRequest <-
            id
            |> PollingClient.isCharacterInBattle
            |> Poll.AfterOk
                (fun x ->
                    GD.Print(x)

                    if x then
                        ArenaNode.Value.GetArena id
                    else
                        DungeonNode.Value.GetDungeon id

                    )

            |> Poll.IgnoreResult
            |> Some
