use std::{fs::File, io::Write};

use card_game_shared::{
    battle::{BattleErrors, ReturnBattle, TakeAction, TurnResponse},
    battle_log::{Action, ActionsDuringTurn, PossibleActions, TriggerTypes},
    characters::{CharacterCreationResponse, CharacterList},
    dungeon::{DungeonLayout, EventProcesed, TileState},
    users::{LoginData, LoginReply, RegisterData},
    ErrorMessage,
};
use schemars::schema_for;
use type_gen::ExternalTypeCollector;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let types_to_generate = vec![
        (schema_for!(ErrorMessage), "ErrorMessage"),
        (schema_for!(BattleErrors), "BattleErrors"),
        (schema_for!(ReturnBattle), "ReturnBattle"),
        (schema_for!(TakeAction), "TakeAction"),
        (schema_for!(TurnResponse), "TurnResponse"),
        (schema_for!(Action), "Action"),
        (schema_for!(ActionsDuringTurn), "ActionsDuringTurn"),
        (schema_for!(PossibleActions), "PossibleActions"),
        (schema_for!(TriggerTypes), "TriggerTypes"),
        (
            schema_for!(CharacterCreationResponse),
            "CharacterCreationResponse",
        ),
        (schema_for!(CharacterList), "CharacterList"),
        (schema_for!(DungeonLayout), "DungeonLayout"),
        (schema_for!(EventProcesed), "EventProcesed"),
        (schema_for!(TileState), "TileState"),
        (schema_for!(LoginData), "LoginData"),
        (schema_for!(LoginReply), "LoginReply"),
        (schema_for!(RegisterData), "RegisterData"),
    ];

    let mut collector = ExternalTypeCollector {};
    for (type_to_generate, location) in types_to_generate {
        let text = type_gen::gen(type_to_generate, &mut collector)
            .unwrap_or_else(|v| panic!("Error while working on : {}.\nError:{:?}", location, v));
        let mut file = File::create(format!(
            "../godot_client/fsharp/card_gamefs/schemas/{}.schema.fs",
            location
        ))?;
        file.write_all(text.as_bytes())?;
    }
    Ok(())
}
