use std::{fs::File, io::Write};

use card_game_shared::{
    battle::{BattleErrors, ReturnBattle, TakeAction, TurnResponse},
    battle_log::{Action, ActionsDuringTurn, PossibleActions, TriggerTypes},
    characters::{CharacterCreationResponse, CharacterList},
    dungeon::{DungeonLayout, EventProcesed, TileState},
    image_map::{ImageUrlWithName, SerializedSpriteSheet},
    users::{LoginData, LoginReply, RegisterData},
    ErrorMessage,
};
use schemars::schema_for;
use type_gen::ExternalTypeCollector;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let types_to_generate = vec![
        (schema_for!(TriggerTypes), "TriggerTypes"),
        (schema_for!(PossibleActions), "PossibleActions"),
        (schema_for!(Action), "Action"),
        (schema_for!(ActionsDuringTurn), "ActionsDuringTurn"),
        (schema_for!(ErrorMessage), "ErrorMessage"),
        (schema_for!(BattleErrors), "BattleErrors"),
        (schema_for!(ReturnBattle), "ReturnBattle"),
        (schema_for!(TakeAction), "TakeAction"),
        (schema_for!(TurnResponse), "TurnResponse"),
        (
            schema_for!(CharacterCreationResponse),
            "CharacterCreationResponse",
        ),
        (schema_for!(CharacterList), "CharacterList"),
        (schema_for!(TileState), "TileState"),
        (schema_for!(DungeonLayout), "DungeonLayout"),
        (schema_for!(EventProcesed), "EventProcesed"),
        (schema_for!(LoginData), "LoginData"),
        (schema_for!(LoginReply), "LoginReply"),
        (schema_for!(RegisterData), "RegisterData"),
        (schema_for!(SerializedSpriteSheet), "SerializedSpriteSheet"),
        (schema_for!(ImageUrlWithName), "ImageUrlWithName"),
    ];

    let mut collector = ExternalTypeCollector::new();
    for (type_to_generate, location) in types_to_generate {
        type_gen::gen(type_to_generate.clone(), &mut collector)
            .unwrap_or_else(|v| {
                println!("Error while working on : {}.\nError:{:?}", location, v);
                println!(
                    "schema to generate:\n {}",
                    serde_json::to_string_pretty(&type_to_generate)
                        .expect("Failed to serialise schema")
                );
                panic!();
            })
            .into_option()
            .map(|v| v.to_owned())
            .map::<Result<_, Box<dyn std::error::Error>>, _>(|text| {
                let mut file = File::create(format!(
                    "../godot_client/fsharp/card_gamefs/schemas/{}.schema.fs",
                    location
                ))?;
                file.write_all(
                    format!(
                        "namespace JsonData \n\n{}\n{}",
                        collector
                            .get_new_external_types()
                            .map(|v| format!("{}\n", v.1))
                            .collect::<String>(),
                        text
                    )
                    .as_bytes(),
                )?;
                Ok(())
            })
            .unwrap_or_else(|| {
                println!(
                    "skipping {}\nIt was already generated as a dependency for another type",
                    location
                );
                Ok(())
            })?;
    }
    Ok(())
}
