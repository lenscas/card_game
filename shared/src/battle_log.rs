#[cfg(feature = "server")]
use rlua::{MetaMethod, UserDataMethods};
#[cfg(feature = "server")]
use tealr::{TealData, TealDataMethods, TealDerive, TypeRepresentation, UserData};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(TealDerive))]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum PossibleActions {
    Card(String),
    Nothing,
}
#[cfg(feature = "server")]
impl TealData for PossibleActions {}

#[cfg_attr(feature = "server", derive(TealDerive))]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum TriggerTypes {
    SmallRuneUser(usize),
    HexaRune(usize),
    SmallRuneDefender(usize),
}
#[cfg(feature = "server")]
impl TealData for TriggerTypes {}

#[cfg_attr(feature = "server", derive(TealDerive))]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Action {
    pub triggered_before: Vec<TriggerTypes>,
    pub taken_action: PossibleActions,
    pub triggered_after: Vec<TriggerTypes>,
}

#[cfg(feature = "server")]
impl TealData for Action {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method_mut("add_trigger_before", |_, me, x| {
            me.triggered_before.push(x);
            Ok(())
        });
        methods.add_method_mut("add_trigger_after", |_, me, x| {
            me.triggered_after.push(x);
            Ok(())
        });
    }
}

#[cfg_attr(feature = "server", derive(TealDerive))]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct ActionsDuringTurn {
    before_turn: Vec<TriggerTypes>,
    first_action: Action,
    second_action: Action,
    after_turn: Vec<TriggerTypes>,
    did_player_go_first: bool,
}

impl ActionsDuringTurn {
    pub fn new(first_action: Action, second_action: Action, did_player_go_first: bool) -> Self {
        Self {
            before_turn: Default::default(),
            first_action,
            second_action,
            after_turn: Default::default(),
            did_player_go_first,
        }
    }
}

#[cfg(feature = "server")]
impl TealData for ActionsDuringTurn {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_function("create_trigger_small_rune_user", |_, x| {
            Ok(TriggerTypes::SmallRuneUser(x))
        });

        methods.add_function("create_trigger_hexa_rune", |_, x| {
            Ok(TriggerTypes::HexaRune(x))
        });
        methods.add_function("create_trigger_small_rune_defender", |_, x| {
            Ok(TriggerTypes::SmallRuneDefender(x))
        });

        methods.add_method_mut("add_before", |_, me, trigger| {
            me.before_turn.push(trigger);
            Ok(())
        });

        methods.add_method_mut("add_after", |_, me, trigger| {
            me.after_turn.push(trigger);
            Ok(())
        });
        methods.add_method_mut("add_first_action", |_, me, x| {
            me.first_action = x;
            Ok(())
        });
        methods.add_method_mut("add_second_action", |_, me, x| {
            me.second_action = x;
            Ok(())
        });
        methods.add_meta_method(MetaMethod::ToString, |_, me, _: ()| Ok(format!("{:?}", me)));
    }
}
