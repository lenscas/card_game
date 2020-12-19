namespace CardGame

open Godot

type DungeonTilesFs() =
    inherit TileMap()

    let mutable drawDungeon: Option<Poll<unit>> = None

    member this._GotDungeon dungeon =
        let dungeon =
            FSharp.Json.Json.deserialize<JsonData.DungeonLayout> dungeon



        let tiles =
            dungeon.tiles
            |> Array.mapi (fun index tile ->
                let x = (index % dungeon.widht) |> float32
                let y = (index / dungeon.widht) |> float32
                let pos = Vector2(x, y)

                (pos, tile))

        GD.Print("got here?")

        drawDungeon <-
            PollingClient.getDungeonTiles ()
            |> poll.MapOk(fun x ->
                x
                |> List.fold (fun y z ->
                    match y with
                    | Ok ls ->
                        match z with
                        | Ok n -> ls |> List.append [ n ] |> Ok
                        | Error z -> Result.Error z

                    | Error x -> Result.Error x) (Ok []))
            |> poll.Flatten
            |> poll.AfterOk(fun x ->
                let rec loop (images: (Image * string) list) count =
                    match images with
                    | head :: tail ->
                        let (image, name) = head
                        this.TileSet.CreateTile count
                        let texture = new ImageTexture()
                        texture.CreateFromImage image
                        this.TileSet.TileSetTexture(count, texture)
                        this.TileSet.TileSetName(count, name)
                        (count + 1) |> loop tail
                    | [] -> ()

                loop x 0

                )
            |> poll.IgnoreResult
            |> poll.After(fun _ ->
                this.Clear()
                for tile in tiles do
                    let (pos, state) = tile

                    let name =
                        match state with
                        | JsonData.TileState.Empty -> None
                        | JsonData.TileState.Hidden -> Some(this.TileSet.FindTileByName("hidden.png"))
                        | JsonData.TileState.Seen x -> Some(this.TileSet.FindTileByName(x))

                    match name with
                    | Some (x) -> this.SetCellv(pos, x)
                    | None -> ()
                GD.Print "done")
            |> poll.AndThen(fun _ ->
                Globals.getCurrentId ()
                |> PollingClient.getPlayerImage
                |> poll.MapOk(fun x ->
                    let location =
                        this.MapToWorld(Vector2((float32 dungeon.player_at.x), float32 dungeon.player_at.y), true)
                        + this.Position
                        + (this.CellSize / Vector2(float32 2.0, float32 2.0))

                    this.EmitSignal("SetPlayerPos", location, x)))
            |> poll.IgnoreResult
            |> Some

        ()

    override this._Process delta = drawDungeon |> poll.TryIgnorePoll
