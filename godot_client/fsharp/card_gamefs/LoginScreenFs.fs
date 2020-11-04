namespace CardGame

open Godot

type LoginScreenFs() as this =
    inherit Control()

    let userNameNode =
        lazy (this.GetNode(new NodePath("UserName")) :?> LineEdit)

    let passwordNode =
        lazy (this.GetNode(new NodePath("Password")) :?> LineEdit)

    member this._OnLoginButtonpressed() =
        let waiter =
            fun () -> this.ToSignal(this.GetTree(), "idle_frame")

        let y =
            BasicClient.connect "www.httpbin.org" 80 false waiter

        GD.Print(y)
        GD.Print("test")

        let x =
            BasicClient.login (userNameNode.Value.Text passwordNode.Value.Text waiter)

        match x with
        | Ok (x) ->
            match x with
            | Ok (x) ->
                GD.Print("double succes")
                GD.Print(System.Text.Encoding.ASCII.GetString(List.toArray x))
            | Result.Error x ->
                GD.Print("Second error")
                GD.Print(x)
        | Result.Error x ->
            GD.Print("First error")
            GD.Print(x)
