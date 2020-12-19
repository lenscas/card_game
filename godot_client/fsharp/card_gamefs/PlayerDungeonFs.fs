namespace CardGame

open Godot

type PlayerDungeonFs() =
    inherit Sprite()

    [<Export>]
    member val Text = "Hello World!" with get, set

    override this._Ready() = GD.Print(this.Text)

    member this._on_SetPlayerPosAndImage (pos: Vector2) (image: Image): unit =
        GD.Print("got here?")
        this.Position <- pos //set the position
        let texture = new ImageTexture() //create a new texture from the given image
        texture.CreateFromImage image //load the image
        this.Texture <- texture //set the sprite texture to the image
