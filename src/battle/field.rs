use super::{deck::Deck, HexaRune, Player};
use crate::{errors::ReturnErrors, util::CastRejection};
use rlua::{Lua, UserData, UserDataMethods};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, PgConnection};
use std::{error::Error, fmt::Display, fs::read_to_string as read_to_string_sync, sync::Arc};
use tokio::fs::read_to_string;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct Field {
    pub(crate) player: Player,
    pub(crate) ai: Player,
    pub(crate) runes: [Option<HexaRune>; 5],
    pub(crate) rune_count: u64,
}

impl Field {
    pub(crate) async fn new(
        player_id: i64,
        con: &mut PoolConnection<PgConnection>,
    ) -> Result<Field, ReturnErrors> {
        let deck = Deck::create_deck_for_player(player_id, con).await?;
        let player = Player {
            life: 20,
            deck,
            mana: 0,
            runes: Default::default(),
            rune_count: 0,
        };
        Ok(Self {
            player: player.clone(),
            ai: player,
            runes: Default::default(),
            rune_count: 0,
        })
    }
    pub async fn process_turn(mut self, chosen_card: usize) -> Result<(Self, bool), ReturnErrors> {
        let card = self.player.get_casted_card(chosen_card)?;
        let lua = Lua::new();
        let engine = read_to_string("./lua/engine.lua").await?;
        let (battle, is_over) = lua.context::<_, Result<_, ReturnErrors>>(move |lua_ctx| {
            let globals = lua_ctx.globals();
            globals.set("battle", self).half_cast()?;
            globals.set("chosenCard", card).half_cast()?;
            let v = lua_ctx.load(&engine).set_name("test?")?.eval()?;
            Ok(v)
        })?;
        Ok((battle, is_over))
    }
}
impl UserData for Field {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("get_ai_card", |_, me, _: ()| {
            let index = 0;
            let item = me.ai.deck.get_card_from_hand(index)?;
            Ok(item)
        });

        methods.add_method("get_ai", |_, me, _: ()| Ok(me.ai.clone()));
        methods.add_method("get_player", |_, me, _: ()| Ok(me.player.clone()));
        methods.add_method("has_ended", |_, me, _: ()| {
            Ok(me.player.life == 0 || me.ai.life == 0)
        });
        methods.add_method_mut("save_ai", |_, me, ai: Player| {
            me.ai = ai;
            Ok(())
        });
        methods.add_method_mut("save_player", |_, me, player: Player| {
            me.player = player;
            Ok(())
        });
        methods.add_method("get_runes", |_, me, _: ()| Ok(me.runes.to_vec()));
        methods.add_method_mut("save_rune", |_, me, (rune, index): (HexaRune, usize)| {
            if let Some(old_rune) = me.runes.get_mut(index).and_then(|v| v.as_mut()) {
                *old_rune = rune;
            } else {
                return Err(SimpleError::new_lua_error(format!(
                    "Error saving rune. Requested index {}, but length is {}.\n Rune : {:?}",
                    index,
                    me.runes.len(),
                    rune
                )));
            }
            Ok(())
        });
        methods.add_method_mut("clean_up_runes", |_, me, _: ()| {
            for v in &mut me.runes {
                if let Some(rune) = v {
                    if rune.config.turns_left == 0 {
                        *v = None;
                    }
                }
            }
            Ok(())
        });
        methods.add_method_mut("add_rune", |_, me, rune_name: String| {
            let mut found = false;
            let as_str = read_to_string_sync(format!(
                "./lua/compiled/hexa_runes/config/{}.lua",
                rune_name
            ))
            .map_err(|v| {
                dbg!(rune_name.clone());
                rlua::Error::ExternalError(Arc::new(v))
            })?;
            let rune = serde_json::from_str(&as_str)
                .map_err(|v| rlua::Error::ExternalError(Arc::new(v)))?;
            let rune = HexaRune::new(me.rune_count, rune, rune_name);
            for v in &mut me.runes {
                if v.is_none() {
                    me.rune_count += 1;
                    *v = Some(rune.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                let mut first: Option<(usize, HexaRune)> = None;
                for (key, v) in me.runes.iter().enumerate() {
                    match &mut first {
                        Some(c) => {
                            if c.1.id < v.as_ref().unwrap().id {
                                first = v.clone().map(|v| (key, v))
                            }
                        }
                        None => first = v.clone().map(|v| (key, v)),
                    };
                }
                let (key, _) = first.unwrap();
                std::mem::replace(&mut me.runes[key], Some(rune));
            }
            Ok(())
        })
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct SimpleError(pub String);
impl SimpleError {
    pub fn new_lua_error(str: String) -> rlua::Error {
        rlua::Error::ExternalError(Arc::new(Self(str)))
    }
}
impl Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for SimpleError {}
