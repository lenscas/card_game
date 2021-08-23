namespace CardGame

open Godot

type DungeonTilesFs() =
    inherit TileMap()

    let mutable drawDungeon: Option<Poll<JsonData.DungeonLayout * (JsonData.TileAction -> int -> unit)>> = None
    let mutable moveInDungeon: Option<Poll<unit>> = None
    let mutable lastDungeon: Option<JsonData.DungeonLayout> = None

    let calcPos index width =
        let x = (index % width) |> float32
        let y = (index / width) |> float32
        Vector2(x, y)

    member this._GotDungeon (dungeon: JsonData.DungeonLayout) (func: JsonData.TileAction -> int -> unit) =

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
            |> Poll.Map(fun () -> (dungeon, func))
            |> Some

        ()

    override this._Input event =
        if moveInDungeon.IsNone then
            drawDungeon
            |> Poll.TryIgnorePeek
                (fun (dungeonLayout, func) ->
                    let (|IsPressed|_|) (key: string) (event: InputEvent) =
                        if event.IsActionPressed key then
                            Some IsPressed
                        else
                            None

                    let res =
                        match event with
                        | IsPressed "ui_left" -> Some(dungeonLayout.player_at.x - 1, dungeonLayout.player_at.y, -1, 0)
                        | IsPressed "ui_right" -> Some(dungeonLayout.player_at.x + 1, dungeonLayout.player_at.y, 1, 0)
                        | IsPressed "ui_up" -> Some(dungeonLayout.player_at.x, dungeonLayout.player_at.y - 1, 0, -1)
                        | IsPressed "ui_down" -> Some(dungeonLayout.player_at.x, dungeonLayout.player_at.y + 1, 0, 1)
                        | _ -> None

                        |> Option.filter (fun (playerX, playerY, _, _) -> (playerY >= 0 && playerX >= 0))
                        |> Option.bind
                            (fun (playerX, playerY, moveX, moveY) ->
                                let tileIndex =
                                    (playerY * dungeonLayout.widht) + playerX

                                if tileIndex < dungeonLayout.tiles.Length then
                                    let tile = dungeonLayout.tiles.[tileIndex]
                                    Some((playerX, playerY, moveX, moveY), tile)
                                else
                                    None)

                    match res with
                    | None
                    | Some (_, JsonData.TileState.Empty) -> ()
                    | Some ((playerX, playerY, moveX, moveY),
                            (JsonData.TileState.Hidden
                            | (JsonData.TileState.Seen _))) ->
                        let oldRequest = drawDungeon
                        drawDungeon <- None

                        moveInDungeon <-
                            Poll.poll {
                                let currentId = Globals.getCurrentId ()
                                let! res = PollingClient.MoveInDungeon currentId { x = moveX; y = moveY }

                                match res with
                                | Ok (Some (JsonData.EventProcesed.Error)) ->
                                    GD.PrintErr("Something has gone wrong")
                                    drawDungeon <- oldRequest
                                    moveInDungeon <- None
                                | Ok (Some (JsonData.EventProcesed.CurrentlyInAction (Some action)))
                                | Ok (Some (JsonData.EventProcesed.Success (Some action))) ->
                                    match action.tile_type with
                                    | JsonData.TileType.Fight -> this.EmitSignal("EnteredBattle")
                                    | _ -> func action currentId
                                | Ok (Some (JsonData.CurrentlyInAction (None))) -> this.EmitSignal("EnteredBattle")
                                | Ok (None) ->
                                    drawDungeon <- oldRequest
                                    moveInDungeon <- None
                                | Result.Error x ->
                                    GD.PrintErr x
                                    drawDungeon <- oldRequest
                                    moveInDungeon <- None
                                | Ok (Some (JsonData.EventProcesed.Success (None))) ->
                                    this.EmitSignal("UpdateDungeon", currentId)
                            }
                            |> Some

                        ())

    override __._Process delta =
        drawDungeon |> Poll.TryIgnorePoll
        moveInDungeon |> Poll.TryIgnorePoll
