global record CardTemplate 
	name : string
	description:string
	cost : number
	speed : number
    id: string
    func : function(CardFuncParams)
    code : string
    is_obtainable: boolean
    is_starting:boolean
end

global record CardFuncParams
    card : Rust.Card
    owner: Rust.Player
    oponent: Rust.Player
    card_list : Rust.Action
    event_list : Rust.ActionsDuringTurn
end