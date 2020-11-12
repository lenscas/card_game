namespace CardGame

module BasicClient =
    open JsonData
    open FSharp.Json
    open Godot
    open System
    open System.Collections.Generic
    open System.Collections
    open System.Diagnostics
    open System.Runtime.ExceptionServices
    open System.Threading
    open System.Threading.Tasks
    open FSharp.Control.Tasks
    open FSharp.Control.Tasks.NonAffine

    let mutable private token: option<string> = None
    let mutable private httpClient = new HTTPClient()

    let rec private pollBuilder checkGoodState afterPoll value =
        if checkGoodState (httpClient.GetStatus()) then
            match httpClient.Poll() with
            | Error.Ok ->
                let res = afterPoll (value)
                pollBuilder checkGoodState afterPoll res
            | x -> Result.Error(x)
        else
            Ok(value)

    let private getRequestValue () =
        let x =
            pollBuilder (fun state -> state = HTTPClient.Status.Body)

        let y =
            x (fun value ->
                    let chunk = httpClient.ReadResponseBodyChunk()
                    match chunk with
                    | [||] ->
                        OS.DelayUsec(1000u)
                        value
                    | x -> List.append value (Array.toList chunk))

        let z = y []
        z


    let private poll x y (wait: unit -> SignalAwaiter2) =
        let rec pollInner z =
            task {
                match httpClient.GetStatus() with
                | HTTPClient.Status.Resolving
                | HTTPClient.Status.Requesting
                | HTTPClient.Status.Connecting ->
                    match httpClient.Poll() with
                    | Error.Ok ->
                        GD.Print "Connecting..."
                        GD.Print "second?"
                        GD.Print "third"
                        let! _ = wait ()
                        return! pollInner (z)

                    | x -> return Result.Error(x)
                | status -> return Result.Ok(z (status))
            }

        match x () with
        | Error.Ok -> pollInner (y)
        | x -> task { return Result.Error(x) }





    let connect host port ssl wait =

        let func =
            poll (fun () -> httpClient.ConnectToHost(host, port, ssl))

        let func2 =
            func (fun x ->
                match x with
                | HTTPClient.Status.Connected -> Ok(())
                | x -> Result.Error(x))

        func2 wait


    // type CustomError<'a> =
    //     | GDError of Error
    //     | Custom of 'a

    // let asGDError err = CustomError.GDError(err)
    // let asCustomError err = CustomError.Custom(err)

    let login (loginData: LoginData) wait =
        task {
            let! x =
                poll (fun () -> httpClient.Request(HTTPClient.Method.Post, "/login", [||], (Json.serialize loginData))) (fun _ ->
                    getRequestValue ()) wait

            match x with
            | Ok (x) ->
                GD.Print("Frist success!")
                match x with
                | Ok (x) ->
                    GD.Print("Second success!")
                    GD.Print(System.Text.Encoding.UTF8.GetString(List.toArray x))
                | Result.Error (x) ->
                    GD.PrintErr("Second error")
                    GD.PrintErr(x)
            | Result.Error (x) ->
                GD.PrintErr("First error")
                GD.PrintErr(x)

            let y =
                Result.bind id x
                //|> Result.mapError asGDError
                |> Result.map List.toArray
                |> Result.map System.Text.Encoding.UTF8.GetString
                |> Result.map (fun x -> Json.deserialize<LoginReply> x)

            return match y with
                   | Ok (x) ->
                       Ok
                           (if x.success then
                               token <- Some(x.token)
                               GD.Print(token)
                               true
                            else
                                false)
                   | Result.Error (x) -> Result.Error(x)
        }
