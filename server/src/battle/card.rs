use rlua::{UserDataMethods,MetaMethod};
use serde::{Deserialize, Serialize};
use tealr::{TealData, TealDataMethods,UserData,TypeRepresentation};

fn default_true() -> bool {
    true
}

#[derive(Deserialize, Serialize, Clone, Debug,UserData,TypeRepresentation)]
pub struct Card {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) speed: u8,
    pub(crate) cost: u64,
    pub(crate) code: String,
    #[serde(default = "default_true")]
    pub(crate) should_reshuffle: bool,
}
impl TealData for Card {
    fn add_methods<'lua, M: TealDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_cost", |_, me, _: ()| Ok(me.cost));
        methods.add_method("get_speed", |_, me, _: ()| Ok(me.speed));
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_code", |_, me, _: ()| Ok(me.code.clone()));
        methods.add_method("get_id", |_, me, _: ()| Ok(me.id.clone()));
        methods.add_meta_method(MetaMethod::ToString, |_,me,_ :()|{
            Ok(format!("{:?}",me))
        })
    }
}
