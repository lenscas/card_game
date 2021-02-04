use card_game::battle::*;
use card_game_shared::battle_log::*;
use tealr::{Direction, TypeWalker};

fn main() {
    let types = TypeWalker::new()
        .process_type::<Field>(Direction::ToLua)
        .process_type::<Card>(Direction::ToLua)
        .process_type::<Player>(Direction::ToLua)
        .process_type::<HexaRune>(Direction::ToLua)
        .process_type::<SmallRune>(Direction::ToLua)
        .process_type::<Action>(Direction::ToLua)
        .process_type::<ActionsDuringTurn>(Direction::ToLua)
        .process_type::<PossibleActions>(Direction::ToLua)
        .process_type::<TriggerTypes>(Direction::ToLua);
    let as_definition_string = types.generate_global("Rust").unwrap();
    println!("{}", as_definition_string);
}
