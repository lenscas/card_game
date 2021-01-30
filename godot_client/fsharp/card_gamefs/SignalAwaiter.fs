namespace CardGame

open System.Runtime.CompilerServices
open Godot

type SignalAwaiter2(wrapped: SignalAwaiter) =
    interface ICriticalNotifyCompletion with
        member this.OnCompleted(cont) = wrapped.OnCompleted(cont)
        member this.UnsafeOnCompleted(cont) = wrapped.OnCompleted(cont)

    member this.IsCompleted = wrapped.IsCompleted

    member this.GetResult() = wrapped.GetResult()

    member this.GetAwaiter() = this
