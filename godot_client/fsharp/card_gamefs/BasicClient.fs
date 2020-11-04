namespace CardGame

module BasicClient =

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


    let login username password wait =
        let x =
            poll (fun () -> httpClient.Request(HTTPClient.Method.Get, "/anything", [||])) (fun _ -> getRequestValue ())
                wait

        x
