namespace CardGame

open Godot

type ControlFs() =
    inherit Godot.Control()

    [<Export>]
    member val Text = "Hello World!" with get, set

    override this._Ready() =
        GD.Print(this.Text)
