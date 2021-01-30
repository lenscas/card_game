namespace CardGame

open Godot

type HandContainerFs() as this =
    inherit Node2D()

    let cardSize = Vector2(101.76331338214f, 150.0f)
    let baseLocation = Vector2(6.4f, 5.00000046875f)


    let getLocationOfCards (cards: List<int * Image>) (characterId: int): List<(ImageTexture * TextureButton)> =
        cards
        |> List.map
            (fun (key, card) ->
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


                let signalParams = new Collections.Array()
                signalParams.Add key |> ignore
                signalParams.Add characterId |> ignore

                match textureButton.Connect("mouse_entered", this, "_OnCardEnter", signalParams) with
                | Error.Ok -> ()
                | x ->
                    GD.PrintErr("Could not connect mouse entered event. Error:")
                    GD.PrintErr(x)

                match textureButton.Connect("mouse_exited", this, "_OnCardLeave", signalParams) with
                | Error.Ok -> ()
                | x ->
                    GD.PrintErr("Could not connect mouse leave event. Error:")
                    GD.PrintErr(x)

                match textureButton.Connect("pressed", this, "_OnCardCast", signalParams) with
                | Error.Ok -> ()
                | x ->
                    GD.PrintErr("Could not connect pressed event. Error:")
                    GD.PrintErr(x)

                this.AddChild(textureButton)
                (texture, textureButton))

    let mutable cards: List<(ImageTexture * TextureButton)> = []
    let mutable highlightedCard: Option<int * TextureRect> = None

    let makeCardHighlight id =
        let (texture, _) = cards.[id]
        let rect = new TextureRect()
        rect.Texture <- texture
        rect.RectSize <- cardSize * 1.2f
        rect.RectPosition <- Vector2(cardSize.x + baseLocation.x + 3.7481698389458f, baseLocation.y)
        this.AddChild(rect)
        id, rect


    member this.SetCards newCards characterId =
        for (_, image) in cards do
            image.QueueFree()

        cards <- getLocationOfCards newCards characterId

        match highlightedCard with
        | Some ((_, x)) -> x.QueueFree()
        | None -> ()


    member this._OnCardEnter(cardId, (_: int)) =
        match highlightedCard with
        | Some (oldId, text) ->
            if oldId = cardId then
                highlightedCard <- cardId |> makeCardHighlight |> Some
        | None -> highlightedCard <- cardId |> makeCardHighlight |> Some

    member this._OnCardLeave(cardId, (_: int)) =
        match highlightedCard with
        | Some (oldId, x) ->
            if oldId = cardId then
                x.QueueFree()
                highlightedCard <- None
        | None ->
            GD.Print("We left a card without having one selected?. Id:")
            GD.Print(cardId)

    member this._OnCardCast((cardId: int), (characterId: int)) =
        this.EmitSignal("PlayCard", cardId, characterId)
