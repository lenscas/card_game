namespace CardGame

open Godot

type DungeonTilesFs() =
    inherit TileMap()

    let mutable drawDungeon: Option<Poll<JsonData.DungeonLayout>> = None
    let mutable moveInDungeon: Option<Poll<unit>> = None
    let mutable lastDungeon: Option<JsonData.DungeonLayout> = None

    let calcPos index width =
        let x = (index % width) |> float32
        let y = (index / width) |> float32
        Vector2(x, y)

    member this._GotDungeon(dungeon: JsonData.DungeonLayout) =

        moveInDungeon <- None

        drawDungeon <-
            PollingClient.getDungeonTiles ()
            |> Poll.MapOk
                (fun x ->
                    x
                    |> List.fold
                        (fun y z ->
                            match y with
                            | Ok ls ->
                                match z with
                                | Ok n -> ls |> List.append [ n ] |> Ok
                                | Error z -> Result.Error z

                            | Error x -> Result.Error x)
                        (Ok []))
            |> Poll.Flatten
            |> Poll.AfterOk
                (fun x ->
                    let rec loop (images: (Image * string) list) count =
                        match images with
                        | (image, name) :: tail ->
                            let nextCount =
                                if this.TileSet.FindTileByName name < 0 then
                                    this.TileSet.CreateTile count
                                    let texture = new ImageTexture()
                                    texture.CreateFromImage image
                                    this.TileSet.TileSetTexture(count, texture)
                                    this.TileSet.TileSetName(count, name)
                                    count + 1
                                else
                                    count

                            loop tail nextCount
                        | [] -> ()

                    this.TileSet.GetLastUnusedTileId() |> loop x

                    )
            |> Poll.IgnoreResult
            |> Poll.After
                (fun _ ->
                    let checker =
                        match lastDungeon with
                        | None ->
                            this.Clear()
                            (fun _ _ -> false)
                        | Some lastDungeon ->
                            if lastDungeon.tiles.Length = dungeon.tiles.Length then
                                (fun i state -> lastDungeon.tiles.[i] = state)
                            else
                                this.Clear()
                                (fun _ _ -> false)

                    for (i, state) in dungeon.tiles |> Seq.indexed do
                        if checker i state |> not then
                            let pos = calcPos i dungeon.widht

                            let name =
                                match state with
                                | JsonData.TileState.Empty -> None
                                | JsonData.TileState.Hidden -> Some(this.TileSet.FindTileByName("hidden.png"))
                                | JsonData.TileState.Seen x -> Some(this.TileSet.FindTileByName(x))

                            match name with
                            | Some (x) -> this.SetCellv(pos, x)
                            | None -> ()

                        GD.Print "done")
            |> Poll.AndThen
                (fun _ ->
                    Globals.getCurrentId ()
                    |> PollingClient.getPlayerImage
                    |> Poll.MapOk
                        (fun x ->
                            let location =
                                this.MapToWorld(
                                    Vector2((float32 dungeon.player_at.x), float32 dungeon.player_at.y),
                                    true
                                )
                                + this.Position
                                + (this.CellSize / Vector2(float32 2.0, float32 2.0))

                            this.EmitSignal("SetPlayerPos", location, x)))
            |> Poll.IgnoreResult
            |> Poll.Map(fun () -> dungeon)
            |> Some

        ()

    override this._Input event =
        if moveInDungeon.IsNone then
            drawDungeon
            |> Poll.TryIgnorePeek
                (fun dungeonLayout ->
                    let res =
                        if event.IsActionPressed("ui_left") then
                            Some(dungeonLayout.player_at.x - 1, dungeonLayout.player_at.y, -1, 0)
                        elif event.IsActionPressed("ui_right") then
                            Some(dungeonLayout.player_at.x + 1, dungeonLayout.player_at.y, 1, 0)
                        elif event.IsActionPressed "ui_up" then
                            Some(dungeonLayout.player_at.x, dungeonLayout.player_at.y - 1, 0, -1)
                        elif event.IsActionPressed "ui_down" then
                            Some(dungeonLayout.player_at.x, dungeonLayout.player_at.y + 1, 0, 1)
                        else
                            None

                    match res with
                    | Some (playerX, playerY, moveX, moveY) ->
                        if playerY >= 0 && playerX >= 0 then
                            let tileIndex =
                                (playerY * dungeonLayout.widht) + playerX

                            if tileIndex < dungeonLayout.tiles.Length then
                                let tile = dungeonLayout.tiles.[tileIndex]

                                match tile with
                                | JsonData.TileState.Empty -> ()
                                | JsonData.TileState.Hidden
                                | JsonData.TileState.Seen _ ->
                                    let oldRequest = drawDungeon
                                    drawDungeon <- None

                                    moveInDungeon <-
                                        Poll.poll {
                                            let currentId = Globals.getCurrentId ()
                                            let! res = PollingClient.MoveInDungeon currentId { x = moveX; y = moveY }

                                            match res with
                                            | Ok (Some (x)) ->
                                                match x with
                                                | JsonData.EventProcesed.Error
                                                | JsonData.EventProcesed.CurrentlyInBattle ->
                                                    //there was a desync, and we showed the dungeon screen while the character is in a battle
                                                    //lets fix this by going to the battle screen.
                                                    this.EmitSignal("EnteredBattle")
                                                | JsonData.EventProcesed.Success true ->
                                                    this.EmitSignal("EnteredBattle")
                                                | JsonData.EventProcesed.Success false ->
                                                    let newLocation: JsonData.BasicVector_for_uint =
                                                        { x = playerX; y = playerY }

                                                    let newDungeon: JsonData.DungeonLayout =
                                                        { dungeonLayout with
                                                              player_at = newLocation }

                                                    moveInDungeon <- None
                                                    this.EmitSignal("UpdateDungeon", currentId)
                                            | Result.Error _
                                            | Ok (None) ->
                                                moveInDungeon <- None
                                                drawDungeon <- oldRequest
                                        }
                                        |> Some

                                ()
                    | None -> ()

                    ())

    override this._Process delta =
        drawDungeon |> Poll.TryIgnorePoll
        moveInDungeon |> Poll.TryIgnorePoll
