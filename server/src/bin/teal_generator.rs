use card_game_shared::battle_log::*;
use tealr::TypeWalker;
use card_game::battle::*;

fn main() {
    let types = TypeWalker::new()
        .proccess_type::<Field>()
        .proccess_type::<Card>()
        .proccess_type::<Player>()
        .proccess_type::<HexaRune>()
        .proccess_type::<SmallRune>()
        .proccess_type::<Action>()
        .proccess_type::<ActionsDuringTurn>()
        .proccess_type::<PossibleActions>()
        .proccess_type::<TriggerTypes>();
    let as_definition_string = types.generate_global("Rust").unwrap();
    println!("{}",as_definition_string);
}
