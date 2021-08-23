namespace CardGame

open Godot

type DungeonCardContainerFs() =
    inherit Node2D()

    let mutable imageRequest: Option<Poll<unit>> = None
    let mutable imageNodes: list<TextureButton> = []

    let cardSize = Vector2(101.76331338214f, 150.0f)
    let baseLocation = Vector2(6.4f, 5.00000046875f)

    member this.Open (action: JsonData.TileAction) (id: int) =
        if action.actions.Length = 0 then
            this.EmitSignal("UpdateDungeon", id)
        else
            imageRequest <-
                action.actions
                |> Array.map (fun x -> x.image)
                |> (fun x ->
                    if action.can_leave then
                        x |> Array.toSeq
                    else
                        x |> Array.toSeq)
                |> Seq.map PollingClient.getImage
                |> Poll.All
                |> Poll.MergeResults
                |> Poll.AfterOk
                    (fun x ->
                        imageNodes
                        |> List.map (fun x -> x.QueueFree())
                        |> ignore

                        imageNodes <-
                            x
                            |> List.mapi
                                (fun key card ->
                                    card.Resize(Mathf.RoundToInt(cardSize.x), Mathf.RoundToInt(cardSize.y))

                                    let recLocation =
                                        Vector2(baseLocation.x, baseLocation.y + (28.0f * float32 (key)))

                                    let texture = new ImageTexture()
                                    texture.CreateFromImage(card)

                                    let textureButton = new TextureButton()
                                    textureButton.TextureNormal <- texture
                                    textureButton.RectSize <- cardSize
                                    textureButton.RectPosition <- recLocation
                                    textureButton.Expand <- true
                                    this.AddChild(textureButton)
                                    textureButton)

                        this.Show())
                |> Poll.IgnoreResult
                |> Some


    override __._Process _ = imageRequest |> Poll.TryIgnorePoll
