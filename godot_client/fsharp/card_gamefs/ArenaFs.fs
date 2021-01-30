namespace CardGame

open Godot
open System

type ArenaFs() as this =
    inherit Node2D()

    let runeContainer =
        lazy (this.GetNode<RuneContainerFs>(new NodePath("RuneContainer")))

    let globalRuneContainer =
        lazy (this.GetNode<RuneContainerFs>(new NodePath("RuneContainer/GlobalRuneContainer")))

    let enemyHp =
        lazy (this.GetNode<Label>(new NodePath("enemy_hp")))

    let enemyMana =
        lazy (this.GetNode<Label>(new NodePath("enemy_mana")))

    let enemySize =
        lazy (this.GetNode<Label>(new NodePath("enemy_size")))

    let playerHp =
        lazy (this.GetNode<Label>(new NodePath("player_mana")))

    let playerMana =
        lazy (this.GetNode<Label>(new NodePath("player_hp")))

    let handContainer =
        lazy (this.GetNode<HandContainerFs>(new NodePath("HandContainer")))


    let calcPoints radius points rotation (offset: float32 -> float32 -> float -> float32 * float32) =
        let steps = 2.0 * Math.PI / float (points)

        let inline halfSquare num =
            let res = LanguagePrimitives.DivideByInt num 2
            res * res



        (seq { 0 .. points })
        |> Seq.map (
            (fun x -> float (x) * steps + rotation)
            >> (fun x -> (radius * Math.Sin(x), radius * Math.Cos(x)))
            >> (fun (x, y) -> (float32 (x), float32 (y)))
            >> (fun (x, y) -> offset x y radius)

        )
        |> Seq.toList

    let mutable arenaRequest = None
    let mutable cardCastRequest = None

    member this.GetArena characterId =
        arenaRequest <-
            Poll.poll {
                let! arena = PollingClient.getBattle characterId

                match arena with
                | Ok (Some (battle), handImages) ->

                    enemyHp.Value.Text <- battle.enemy_hp.ToString()
                    enemyMana.Value.Text <- battle.enemy_mana.ToString()
                    enemySize.Value.Text <- battle.enemy_hand_size.ToString()
                    playerHp.Value.Text <- battle.player_hp.ToString()
                    playerMana.Value.Text <- battle.mana.ToString()

                    runeContainer.Value.Runes <- calcPoints 230.2875549 8 10. (fun x y _ -> (x, y))

                    globalRuneContainer.Value.Runes <- calcPoints 134.334407028 5 0. (fun x y _ -> (x, y))

                    handContainer.Value.SetCards handImages characterId

                    this.Show()
                | Ok (None, _) -> GD.PrintErr("Failed to deserialize the arena")
                | Result.Error x ->
                    GD.PrintErr("Failed to make request. Error:")
                    GD.PrintErr(x)

                return ()
            }
            |> Some

        ()

    member this.onPlayCard(cardId, characterId) =
        cardCastRequest <-
            Poll.poll {
                let! res = PollingClient.castCard cardId characterId
                cardCastRequest <- None

                match res with
                | Result.Error x -> GD.PrintErr(x)
                | Ok (Some (JsonData.TurnResponse.NextTurn x)) ->
                    GD.Print(x)
                    this.GetArena characterId
                | Ok (Some (JsonData.TurnResponse.Error x)) -> GD.PrintErr(x)
                | Ok (Some JsonData.TurnResponse.Done) ->
                    GD.Print("Done")
                    this.EmitSignal("BattleEnded")
                | Ok None -> GD.PrintErr("Failed to deserialize!")
            }
            |> Some



    override this._Process delta =
        globalRuneContainer.Value.Rotate(0.1f * delta)

        match cardCastRequest, arenaRequest with
        | (Some cardCastRequest), _ -> cardCastRequest |> Poll.ignorePoll
        | _, Some arenaRequest -> arenaRequest |> Poll.ignorePoll
        | _, _ -> ()
