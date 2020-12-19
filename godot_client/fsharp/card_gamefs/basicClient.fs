namespace CardGame

open JsonData
open System.Collections.Generic

module PollingClient =
    open Godot
    open FSharp.Json

    let mutable private token: string option = None

    let mutable private url: string option = None
    let mutable private port: int option = None
    let mutable private usessl: bool option = None

    let clients = new List<HTTPClient>()

    //let private client = new HTTPClient()

    let waitCheck (client: HTTPClient) =
        match client.GetStatus() with
        | HTTPClient.Status.Resolving
        | HTTPClient.Status.Requesting
        | HTTPClient.Status.Connecting ->
            match client.Poll() with
            | Error.Ok -> NotYet

            | x -> Got(Result.Error x)
        | status -> Got(Ok status)

    let fromGodotError x y =
        match y with
        | Error.Ok -> x ()
        | x -> Result.Error(x)

    let private getConnection (client: HTTPClient) host port ssl =
        client.ConnectToHost(host, port, ssl, false)
        |> fromGodotError (fun () -> Ok(Poll(fun () -> waitCheck client)))
        |> Poll.FromResult
        |> poll.Flatten
        |> poll.MapOk(fun _ -> client)

    let private findConnection () =
        let found =
            clients
            |> Seq.tryFind (fun x -> x.GetStatus() = HTTPClient.Status.Connected)

        match found with
        | Some x -> x |> Ok |> Poll.Ready
        | None ->
            let newClient = new HTTPClient()
            (getConnection newClient url.Value port.Value usessl.Value)
            |> poll.AfterOk(fun x -> x.GetStatus() |> ignore)

    let createUrl parts =
        parts |> List.fold (fun r s -> r + "/" + s) ""








    let connect host port1 ssl =
        url <- Some host
        port <- Some port1
        usessl <- Some ssl
        findConnection ()
        |> poll.MapOk(fun x -> x.GetStatus())

    // client.ConnectToHost(host, port1, ssl, false)
    // |> fromGodotError (fun () -> Ok(Poll waitCheck))
    // |> Poll.FromResult
    // |> poll.Flatten

    let bareRequest urlPart method (data: option<string>) =
        findConnection ()
        |> poll.AndThenOk(fun client ->
            client.Request
                (method,
                 urlPart,
                 [| if token.IsSome
                    then yield "authorization_token: " + token.Value
                    yield "Accept: application/json"
                    yield "Content-Type: application/json" |],
                 data |> Option.toObj)
            |> fromGodotError (fun () -> Ok(Poll(fun () -> waitCheck client)))
            |> Poll.FromResult
            |> poll.Flatten
            |> poll.AndThenOk(fun _ ->
                let mutable res: byte list = []
                Poll(fun () ->
                    match client.GetStatus() with
                    | HTTPClient.Status.Resolving
                    | HTTPClient.Status.Requesting
                    | HTTPClient.Status.Connecting -> NotYet
                    | HTTPClient.Status.Body ->
                        res <-
                            client.ReadResponseBodyChunk()
                            |> Array.toList
                            |> List.append res
                        NotYet
                    | _ -> Got(Ok(res))))
            |> poll.MapOk List.toArray)

    let requestSerialized<'T> urlPart method (data: option<string>) =
        (bareRequest urlPart method data)
        |> poll.MapOk System.Text.Encoding.UTF8.GetString
        |> poll.MapOk(fun x ->
            try
                Some(Json.deserialize<'T> x)
            with _ -> None)

    let getImage url =
        (bareRequest url HTTPClient.Method.Get None)
        |> poll.MapOk(fun x ->
            let image = new Image()
            image.LoadPngFromBuffer(x)
            |> fromGodotError (fun _ -> Ok(image))

            )
        |> poll.Flatten

    let request<'T, 'A> urlPart method (data: Option<'A>) =
        data
        |> Option.map (Json.serialize)
        |> requestSerialized urlPart method




    let post<'T, 'A> (urlPart: string) (data: 'A) =
        data
        |> Some
        |> request<'T, 'A> urlPart HTTPClient.Method.Post

    let emptyPost<'T> urlPart =
        requestSerialized<'T> urlPart HTTPClient.Method.Post None

    let get<'T> urlPart =
        requestSerialized<'T> urlPart HTTPClient.Method.Get None


    let login username password =
        { username = username
          password = password }
        |> post<JsonData.LoginReply, JsonData.LoginData> "/login"
        |> poll.Map(fun x ->
            match x with
            | Ok (x) ->
                match x with
                | Some (x) ->
                    if x.success then token <- Some(x.token)
                    x.success
                | None -> false
            | x -> false)

    let characeterSelect () = get<CharacterList> "/characters"

    let createCharacter () =
        emptyPost<CharacterCreationResponse> "/characters"
        |> poll.MapOk(fun x -> x |> Option.map (fun x -> x.id))

    let isCharacterInBattle (id: int) =
        [ "characters"; id.ToString() ]
        |> createUrl
        |> get<bool>
        |> poll.MapOk(fun x -> x |> Option.defaultValue false)

    let getDungeon (charId: int) =
        [ "dungeon"; charId.ToString() ]
        |> createUrl
        |> get<JsonData.DungeonLayout>

    let getPlayerImage (charId: int) =
        [ "assets"; "player.png" ]
        |> createUrl
        |> getImage

    let getDungeonTiles () =
        [ "dungeon"; "tiles"; "list" ]
        |> createUrl
        |> get<JsonData.ImageUrlWithName []>
        |> poll.AndThenOk(fun x ->
            match x with
            | Some x ->
                let getPictureList =
                    new List<Poll<Result<Image * string, Error>>>()

                for y in x do
                    y.url
                    |> getImage
                    |> poll.MapOk(fun x -> (x, y.name))
                    |> getPictureList.Add

                let p =
                    getPictureList |> Seq.toArray |> poll.All

                p

            | None -> Poll.Ready([])

            |> poll.Map Ok)


// getImage
// |> poll.AndThenOk(fun x ->
//     [ "dungeon"; "tiles"; "map" ]
//     |> createUrl
//     |> get<JsonData.SerializedSpriteSheet>
//     |> poll.MapOk(fun y ->
//         let image = new Image()
//         image.LoadPngFromBuffer(x)
//         |> fromGodotError (fun _ -> Ok(image, y))))
// |> poll.Flatten
