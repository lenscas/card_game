namespace CardGame

open Godot
open JsonData
open System.Threading.Tasks
open FSharp.Json
open FSharp.Json.Json

type LoginScreenFs() as this =
    inherit Control()

    let userNameNode =
        lazy (this.GetNode(new NodePath("UserName")) :?> LineEdit)

    let passwordNode =
        lazy (this.GetNode(new NodePath("Password")) :?> LineEdit)

    let mutable alreadyRunning = false

    member this._OnLoginButtonpressed() =
        if alreadyRunning then
            ()
        else
            alreadyRunning <- true

            let data: JsonData.LoginData =
                { username = userNameNode.Value.Text
                  password = passwordNode.Value.Text }

            async {
                newBasicClient.connect "http://127.0.0.1:3030"


                let! x = newBasicClient.login data

                GD.Print(x)

                alreadyRunning <- false

                ()
            }
            |> Async.StartAsTask
            |> ignore
