namespace CardGame

open Godot

type RuneContainerFs() =
    inherit Node2D()

    let mutable runes = None

    member this.Redraw = this.Update

    member this.Runes
        with set (value) =
            runes <- Some(value)
            this.Update()

    override this._Draw() =
        match runes with
        | None -> ()
        | Some (runes) ->
            for (x, y) in runes do
                this.DrawCircle(Vector2(x, y), (float32 26.2371888726f), Color.Color8(byte 255, byte 255, byte 255))
