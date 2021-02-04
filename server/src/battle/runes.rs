use serde::{Deserialize, Serialize};

use tealr::{
    rlu::{TealData, TealDataMethods},
    TealDerive, TypeName, UserData,
};

#[derive(Clone, Deserialize, Serialize, Debug, TealDerive)]
pub struct RawRune {
    pub(crate) turns_left: u64,
}

impl TealData for RawRune {}

#[derive(Clone, Deserialize, Serialize, Debug, TealDerive)]
pub struct HexaRune {
    pub(crate) config: RawRune,
    pub(crate) name: String,
    pub(crate) id: u64,
}

impl HexaRune {
    pub(crate) fn new(id: u64, config: RawRune, name: String) -> Self {
        HexaRune { id, config, name }
    }
}

impl TealData for HexaRune {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_turns_left", |_, me, _: ()| Ok(me.config.turns_left));
        methods.add_method("id", |_, me, _: ()| Ok(me.id));

        methods.add_method_mut("dec_turns_left", |_, me, _: ()| {
            me.config.turns_left -= 1;
            Ok(me.config.turns_left)
        });
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, TealDerive)]
pub struct SmallRune {
    pub(crate) config: RawRune,
    pub(crate) name: String,
    pub(crate) id: u64,
}

impl SmallRune {
    pub(crate) fn new(id: u64, config: RawRune, name: String) -> Self {
        SmallRune { id, config, name }
    }
}

impl TealData for SmallRune {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_turns_left", |_, me, _: ()| Ok(me.config.turns_left));
        methods.add_method("id", |_, me, _: ()| Ok(me.id));

        methods.add_method_mut("dec_turns_left", |_, me, _: ()| {
            me.config.turns_left -= 1;
            Ok(me.config.turns_left)
        });
    }
}
