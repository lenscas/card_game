namespace CardGame

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

type PollResult<'a> =
    | Got of 'a
    | NotYet

module PollHelper =
    let isDone res =
        match res with
        | Got (_) -> true
        | NotYet -> false

    let isNotDone res = isDone res |> not

    let toOption res =
        match res with
        | Got (x) -> Some(x)
        | NotYet -> None

type Poll<'T>(func: unit -> PollResult<'T>) =
    let mutable funcResult: PollResult<'T> = NotYet

    member this.Poll() =
        match funcResult with
        | Got (x) -> Got(x)
        | NotYet ->
            let res = func ()
            match res with
            | Got (x) ->
                funcResult <- Got(x)
                res
            | x -> x

    member this.Force(waiter: unit -> SignalAwaiter2) =
        task {
            let mutable value: option<'T> = None
            let mutable breakOut = false
            while not breakOut do
                let shouldWait =
                    match this.Poll() with
                    | Got (x) ->
                        value <- Some(x)
                        breakOut <- true
                        false
                    | NotYet -> true

                if shouldWait then
                    let! _ = waiter ()
                    ()
                else
                    ()

            return value |> Option.get
        }

    static member FromResult<'T, 'B>(res: Result<Poll<'T>, 'B>): (Poll<Result<'T, 'B>>) =
        new Poll<Result<'T, 'B>>(fun () ->
        match res with
        | Ok (x) ->
            match x.Poll() with
            | Got (z) -> Got(Ok(z))
            | NotYet -> NotYet
        | Result.Error (x) -> Got(Result.Error(x)))

    static member Ready<'T> res =
        new Poll<'T>(fun () -> PollResult.Got(res))

module poll =
    let AndThen<'A, 'T> (func: 'A -> Poll<'T>) (poll1: Poll<'A>) =
        let mutable otherPoll: option<Poll<'T>> = None

        new Poll<'T>(fun () ->
        match otherPoll with
        | Some (x) -> x.Poll()
        | None ->
            match poll1.Poll() with
            | Got (x) ->
                otherPoll <- Some(func (x))
                NotYet
            | NotYet -> NotYet)

    let AndThenOk<'A, 'ERR, 'B> (func: 'A -> Poll<Result<'B, 'ERR>>) (poll1: Poll<Result<'A, 'ERR>>) =
        let mutable otherPoll: option<Poll<Result<'B, 'ERR>>> = None

        new Poll<Result<'B, 'ERR>>(fun () ->
        match otherPoll with
        | Some (x) -> x.Poll()
        | None ->
            match poll1.Poll() with
            | Got (x) ->
                match x with
                | Ok (x) -> otherPoll <- Some(func (x))
                | Result.Error (x) -> otherPoll <- Some(Poll(fun () -> Got(Result.Error(x))))
                NotYet
            | NotYet -> NotYet)

    let Map<'A, 'T> (func: 'A -> 'T) (poll1: Poll<'A>) =
        poll1
        |> AndThen(fun x -> Poll(fun () -> Got(func (x))))

    let MapOk<'A, 'T, 'Err> (func: 'A -> 'T) (poll: Poll<Result<'A, 'Err>>) =
        poll
        |> Map(fun x ->
            match x with
            | Ok (x) -> Ok(func (x))
            | Result.Error (x) -> Result.Error(x))

    let After func poll =
        poll
        |> Map(fun x ->
            func (x)
            x)

    let AfterOk func poll =
        poll
        |> After(fun x ->
            match x with
            | Result.Error (_) -> ()
            | Ok (x) -> func (x))

    let Flatten<'T, 'ERR> (poll: Poll<Result<Result<'T, 'ERR>, 'ERR>>) =
        poll
        |> Map(fun x ->
            match x with
            | Ok (x) -> x
            | Result.Error (x) -> Result.Error x)

    let IgnoreResult poll = poll |> Map ignore

    let TryPoll<'T, 'A> (func: 'T -> 'A) (poll: Option<Poll<'T>>) =
        match poll with
        | Some (x) ->
            match x.Poll() with
            | Got (x) -> Some(func (x))
            | NotYet -> None
        | None -> None

    let ignorePoll<'T> (poll: Poll<'T>) = poll.Poll() |> ignore
    let TryIgnorePoll<'T> (poll: Option<Poll<'T>>) = poll |> TryPoll ignore |> ignore

    let All<'T> (polls: Poll<'T> []): Poll<'T list> =
        Poll<'T list>(fun () ->
            let rec loop (polls: Poll<'T> list) (state: PollResult<'T list>): PollResult<'T list> =
                match polls with
                | top :: tail ->
                    let res = top.Poll()

                    let newState =
                        match res with
                        | Got x ->
                            match state with
                            | Got y -> y |> List.append [ x ] |> Got
                            | NotYet -> NotYet

                        | NotYet -> NotYet

                    loop tail newState
                | [] -> state

            [] |> Got |> loop (polls |> Array.toList)

            // let res = polls |> Array.map (fun x -> x.Poll())

            // let rec loop (polls: PollResult<'T> list) (state: PollResult<'T list>): PollResult<'T list> =
            //     match state with
            //     | PollResult.NotYet -> NotYet
            //     | PollResult.Got items ->
            //         match polls with
            //         | [] -> state
            //         | top :: tail ->
            //             match top with
            //             | PollResult.NotYet -> loop [] PollResult.NotYet
            //             | PollResult.Got x ->
            //                 items
            //                 |> List.append [ x ]
            //                 |> PollResult.Got
            //                 |> loop tail

            // match res |> Array.toList with
            // | head :: tail ->
            //     match head with
            //     | PollResult.Got _ -> head |> loop tail
            //     | PollResult.NotYet -> PollResult.NotYet

            // | [] -> PollResult.NotYet
            )
