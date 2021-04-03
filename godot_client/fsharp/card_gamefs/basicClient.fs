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
    let mutable private checkedCache = false
    let mutable private memCache = new Dictionary<string, Image>()
    let mutable private imagesStored = new HashSet<string>()
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
        | x ->
            GD.PrintErr(x)
            Result.Error(x)

    let private getConnection (client: HTTPClient) host port ssl =
        client.ConnectToHost(host, port, ssl, false)
        |> fromGodotError (fun () -> Ok(Poll(fun () -> waitCheck client)))
        |> Poll.FromResult
        |> Poll.Flatten
        |> Poll.MapOk(fun _ -> client)

    let private findConnection () =
        let found =
            clients
            |> Seq.tryFind (fun x -> x.GetStatus() = HTTPClient.Status.Connected)

        match found with
        | Some x -> x |> Ok |> Poll.Ready
        | None ->
            let newClient = new HTTPClient()

            (getConnection newClient url.Value port.Value usessl.Value)
            |> Poll.AfterOk(fun x -> x.GetStatus() |> ignore)

    let createUrl parts =
        parts |> List.fold (fun r s -> r + "/" + s) ""


    let connect host port1 ssl =
        url <- Some host
        port <- Some port1
        usessl <- Some ssl

        findConnection ()
        |> Poll.MapOk(fun x -> x.GetStatus())

    // client.ConnectToHost(host, port1, ssl, false)
    // |> fromGodotError (fun () -> Ok(Poll waitCheck))
    // |> Poll.FromResult
    // |> poll.Flatten

    let bareRequest urlPart method (data: option<string>) =
        findConnection ()
        |> Poll.AndThenOk
            (fun client ->
                client.Request(
                    method,
                    urlPart,
                    [| if token.IsSome then
                           yield "authorization_token: " + token.Value
                       yield "Accept: application/json"
                       yield "Content-Type: application/json" |],
                    data |> Option.toObj
                )
                |> fromGodotError (fun () -> Ok(Poll(fun () -> waitCheck client)))
                |> Poll.FromResult
                |> Poll.Flatten
                |> Poll.AndThenOk
                    (fun _ ->
                        let mutable res: byte list = []

                        Poll
                            (fun () ->
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
                |> Poll.MapOk List.toArray)

    let requestSerialized<'T> urlPart method (data: option<string>) =
        (bareRequest urlPart method data)
        |> Poll.MapOk System.Text.Encoding.UTF8.GetString
        |> Poll.MapOk
            (fun x ->
                try
                    Some(Json.deserialize<'T> x)
                with err ->
                    GD.PrintErr("Problem during deserialization. Error:")
                    GD.PrintErr(err)
                    None

                )

    let getBool urlPart =
        (bareRequest urlPart HTTPClient.Method.Get None)
        |> Poll.MapOk System.Text.Encoding.UTF8.GetString
        |> Poll.MapOk
            (fun x ->
                GD.Print("got back:")
                GD.Print(x)

                let trimmed = x.Trim().ToLower()

                if trimmed = "false" then Some false
                elif trimmed = "true" then Some true
                else None)

    let checkCacheDate () =
        let url =
            createUrl [ "assets"
                        "cards"
                        "generation_time.txt" ]

        bareRequest url HTTPClient.Method.Get None
        |> Poll.MapOk System.Text.Encoding.UTF8.GetString
        |> Poll.Map
            (fun y ->
                match y with
                | Result.Error x -> Result.Error x
                | Ok y ->
                    let file = new File()

                    let res =
                        if file.FileExists("user://last_update_time.txt") then
                            let res =
                                file.Open("user://last_update_time.txt", File.ModeFlags.Read)
                                |> fromGodotError (fun () -> Ok(file.GetAsText()))
                                |> Result.map (fun x -> x = y)

                            file.Close()
                            res
                        else
                            Ok false

                    file.Open("user://last_update_time.txt", File.ModeFlags.Write)
                    |> fromGodotError (fun _ -> Ok(file.StoreString y))
                    |> ignore

                    res)













    let readCachedImages () =
        let file = new File()

        if file.FileExists("user://cached_images.json") then
            let x =
                file.Open("user://cached_images.json", File.ModeFlags.Read)
                |> fromGodotError (fun _ -> Ok(file.GetAsText()))
                |> Result.map
                    (fun x ->
                        let arr =
                            try
                                Json.deserialize x
                            with err ->
                                GD.PrintErr(err)
                                []

                        let set = new HashSet<_>()

                        for x in arr do
                            set.Add(x) |> ignore

                        set)

            match x with
            | Ok x -> x
            | Result.Error _ -> new HashSet<_>()
        else
            new HashSet<string>()

    let tryGetImageFromCache url =
        let (found, image) = memCache.TryGetValue(url)

        if found then
            Some image
        else if imagesStored.Contains url then
            let img = new Image()

            match img.Load("user://" + url) with
            | Error.Ok -> Some img
            | _ -> None
        else
            None

    let initCache () =
        if not checkedCache then
            checkCacheDate ()
            |> Poll.AfterOk(fun x -> checkedCache <- true)
            |> Poll.AfterOk
                (fun x ->
                    let imageList = readCachedImages ()

                    imagesStored <-
                        if not x then
                            let dir = new Directory()

                            for v in imageList do
                                dir.Remove("user://" + v) |> ignore

                            new HashSet<_>()

                        else
                            imageList

                    )
            |> Poll.Map ignore

        else
            Poll.Ready()



    let getImage url =
        let image = tryGetImageFromCache url

        match image with
        | Some x -> x |> Ok |> Poll.Ready
        | None ->
            (bareRequest url HTTPClient.Method.Get None)
            |> Poll.MapOk
                (fun x ->
                    let image = new Image()

                    image.LoadPngFromBuffer(x)
                    |> fromGodotError
                        (fun _ ->
                            let urlArr = (url.Split '/')

                            let (_, backToStr) =
                                urlArr
                                |> Array.fold
                                    (fun (i: int, s: string) y ->

                                        if i = urlArr.Length - 1 then
                                            (i + 1, s)
                                        else
                                            (i + 1, s + "/" + y))
                                    (0, "user://")

                            let dir = new Directory()

                            dir.MakeDirRecursive(backToStr)
                            |> fromGodotError (ignore >> Ok)
                            |> ignore

                            match image.SavePng("user://" + url) with
                            | Error.Ok ->
                                imagesStored.Add(url) |> ignore
                                let file = new File()

                                file.Open("user://cached_images.json", File.ModeFlags.Write)
                                |> fromGodotError
                                    (fun _ -> Ok(file.StoreString(imagesStored |> Seq.toArray |> Json.serialize)))
                                |> ignore

                                file.Close()

                            | _ -> ()

                            memCache.Add(url, image)
                            Ok(image))

                    )
            |> Poll.Flatten

    let request<'T, 'A> urlPart method (data: Option<'A>) =
        data
        |> Option.map (Json.serialize)
        |> requestSerialized<'T> urlPart method




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
        |> Poll.Map
            (fun x ->
                match x with
                | Ok (x) ->
                    match x with
                    | Some (x) ->
                        if x.success then token <- Some(x.token)
                        x.success
                    | None -> false
                | x -> false)
        |> Poll.AndThen(fun x -> initCache () |> Poll.Map(fun _ -> x))

    let characeterSelect () = get<CharacterList> "/characters"

    let createCharacter () =
        emptyPost<CharacterCreationResponse> "/characters"
        |> Poll.MapOk(fun x -> x |> Option.map (fun x -> x.id))

    let isCharacterInBattle (id: int) =
        [ "characters"; id.ToString() ]
        |> createUrl
        |> getBool
        |> Poll.MapOk(Option.defaultValue false >> not)

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
        |> Poll.AndThenOk
            (fun x ->
                match x with
                | Some x ->
                    let getPictureList =
                        new List<Poll<Result<Image * string, Error>>>()

                    for y in x do
                        y.url
                        |> getImage
                        |> Poll.MapOk(fun x -> (x, y.name))
                        |> getPictureList.Add

                    let p =
                        getPictureList |> Seq.toArray |> Poll.All

                    p

                | None -> Poll.Ready([])

                |> Poll.Map Ok)

    let MoveInDungeon (charId: int) (newLocation: JsonData.BasicVector_for_uint) =
        let url =
            createUrl [ "dungeon"
                        charId.ToString()
                        "move" ]

        post<JsonData.EventProcesed, _> url newLocation

    let getBattle (characterId: int) =
        [ "battle"; characterId.ToString() ]
        |> createUrl
        |> get<JsonData.ReturnBattle>
        |> Poll.AndThenOk
            (fun x ->
                match x with
                | Some (x) ->
                    x.hand
                    |> Array.mapi
                        (fun i y ->

                            [ "assets"; "cards"; y + ".png" ]
                            |> createUrl
                            |> getImage
                            |> Poll.MapOk(fun x -> i, x))
                    |> Poll.All
                    |> Poll.Map
                        (fun x ->
                            let rec func
                                (l: list<Result<int * Image, Error>>)
                                (state: Option<list<int * Image>>)
                                : Result<list<int * Image>, Error> =
                                match l with
                                | [] ->
                                    match state with
                                    | Some y -> Ok y
                                    | None -> Ok []
                                | head :: tail ->
                                    match head with
                                    | Result.Error y -> Result.Error y
                                    | Ok y ->
                                        match state with
                                        | None -> [ y ] |> Some |> func tail
                                        | Some z -> y :: z |> Some |> func tail

                            func x None)
                | None -> [] |> Ok |> Poll.Ready
                |> Poll.MapOk(fun y -> (x, y)))

    let castCard (card: int) (character: int) =
        let url = [ "battle" ] |> createUrl

        post<JsonData.TurnResponse, _>
            url
            { character_id = character
              play_card = card }
        |> Poll.MapOk(Option.defaultValue TurnResponse.Done >> Some)
