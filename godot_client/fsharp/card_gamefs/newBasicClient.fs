namespace CardGame

open Godot

module newBasicClient =
    open System.Net.Http
    open JsonData
    open FSharp.Json

    let client = new HttpClient()
    let mutable baseurl: option<string> = None
    let mutable token: option<string> = None
    let connect url = baseurl <- Some(url)

    let login (logindata: LoginData) =
        GD.Print("is it at least trying something?")
        async {
            GD.Print("got here?")
            let data = Json.serialize logindata

            use content =
                new StringContent(data, System.Text.Encoding.UTF8, "application/json")

            GD.Print("made content?")
            let! response =
                client.PostAsync(Option.get baseurl + "/login", content)
                |> Async.AwaitTask

            GD.Print("Got a response")

            let! body =
                response.Content.ReadAsStringAsync()
                |> Async.AwaitTask

            GD.Print("time to deserialize")

            let reply = Json.deserialize<LoginReply> body

            GD.Print("deserialized")
            if reply.success then token <- Some(reply.token)

            GD.Print(reply.token)

            return reply.success
        }
