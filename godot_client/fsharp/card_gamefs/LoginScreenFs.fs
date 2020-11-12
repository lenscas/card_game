namespace CardGame

open Godot
open JsonData
open FSharp.Control.Tasks.Affine

type LoginScreenFs() as this =
    inherit Control()

    let userNameNode =
        lazy (this.GetNode(new NodePath("UserName")) :?> LineEdit)

    let passwordNode =
        lazy (this.GetNode(new NodePath("Password")) :?> LineEdit)

    member this._OnLoginButtonpressed() =
        let waiter =
            fun () () -> SignalAwaiter2(this.ToSignal(this.GetTree(), "idle_frame"))

        task {
            let firstWaiter = waiter ()
            let! y = BasicClient.connect "127.0.0.1" 3030 false firstWaiter
            GD.Print(y)
            let secondWaiter = waiter ()

            let data =
                { password = passwordNode.Value.Text
                  username = userNameNode.Value.Text }

            let! x = BasicClient.login data secondWaiter

            match x with
            | Ok (x) ->
                GD.Print("success!")
                GD.Print(x)
            | Result.Error x ->
                GD.Print("Error")
                GD.Print(x)

        }
