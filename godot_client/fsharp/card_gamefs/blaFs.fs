namespace CardGame

open Godot

type blaFs() =
    inherit Node()

    [<Export>]
    member val Text = "Hello World!" with get, set

    override this._Ready() = GD.Print(this.Text)
