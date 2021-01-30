namespace CardGame

open Godot

module newBasicClient =
    open System.Net.Http
    open JsonData
    open FSharp.Json

    let client = new HttpClient()
    let mutable baseurl: option<string> = None
    let mutable token: option<string> = None
    let connect (url: string) = baseurl <- Some((url.TrimEnd '/') + "/")

    let createUrl parts =
        parts
        |> List.fold (fun r s -> r + s + "/") (baseurl |> Option.get)

    let createHeader = []

    let makePost<'T> parts data =
        async {
            let url = createUrl parts

            let content =
                match data with
                | Some (x) ->
                    new StringContent((Json.serialize x), System.Text.Encoding.UTF8, "application/json") :> HttpContent
                | None -> new ByteArrayContent([||]) :> HttpContent

            let! response = client.PostAsync(url, content) |> Async.AwaitTask

            let! body =
                response.Content.ReadAsStringAsync()
                |> Async.AwaitTask

            return Json.deserialize<'T> body
        }

    let makeGet<'T> parts =
        async {
            let! response =
                parts
                |> createUrl
                |> client.GetAsync
                |> Async.AwaitTask

            let! body =
                response.Content.ReadAsStringAsync()
                |> Async.AwaitTask

            return Json.deserialize<'T> body
        }

    let login (logindata: LoginData) =
        async {
            let data = Json.serialize logindata

            use content =
                new StringContent(data, System.Text.Encoding.UTF8, "application/json")

            let! response =
                client.PostAsync(createUrl [ "login" ], content)
                |> Async.AwaitTask

            GD.Print("Got a response")

            let! body =
                response.Content.ReadAsStringAsync()
                |> Async.AwaitTask

            GD.Print("time to deserialize")

            let reply = Json.deserialize<LoginReply> body

            GD.Print("deserialized")

            if reply.success then
                token <- Some(reply.token)
                client.DefaultRequestHeaders.Add("authorization_token", reply.token)

            GD.Print(reply.token)

            return reply.success
        }

    let getCharacterList () =
        async {
            let x = (createUrl [ "characters" ])
            let! res = client.GetAsync x |> Async.AwaitTask

            let! body = res.Content.ReadAsStringAsync() |> Async.AwaitTask

            return Json.deserialize<CharacterList> body

        }

    let createCharacter () =
        async {
            let! res =
                client.PostAsync((createUrl [ "characters" ]), new ByteArrayContent([||]))
                |> Async.AwaitTask

            let! body = res.Content.ReadAsStringAsync() |> Async.AwaitTask

            return (Json.deserialize<CharacterCreationResponse> body).id
        }

    let isCharacterInBattle (id: int64) =
        async { return! makeGet<bool> [ "characters"; id.ToString() ] }
