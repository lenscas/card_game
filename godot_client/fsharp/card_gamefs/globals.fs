namespace CardGame

module Globals =
    let mutable private currentId = None

    let SetCurrentId (id: int) = currentId <- Some(id)
    let getCurrentId () = currentId |> Option.get
