use rlua::{UserData, UserDataMethods};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct RawRune {
    pub(crate) turns_left: u64,
}

impl UserData for RawRune {}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct HexaRune {
    pub(crate) config: RawRune,
    name: String,
    id: u64,
}

impl HexaRune {
    pub(crate) fn new(id: u64, config: RawRune, name: String) -> Self {
        HexaRune { id, config, name }
    }
}

impl UserData for HexaRune {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_turns_left", |_, me, _: ()| Ok(me.config.turns_left));

        methods.add_method_mut("dec_turns_left", |_, me, _: ()| {
            Ok(me.config.turns_left -= 1)
        });
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct SmallRune {
    pub(crate) config: RawRune,
    pub(crate) name: String,
    pub(crate) id: u64,
}

impl SmallRune {
    pub(crate) fn new(id: u64, config: RawRune, name: String) -> Self {
        SmallRune { id, config, name }
    }
}

impl UserData for SmallRune {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_turns_left", |_, me, _: ()| Ok(me.config.turns_left));

        methods.add_method_mut("dec_turns_left", |_, me, _: ()| {
            Ok(me.config.turns_left -= 1)
        });
    }
}
