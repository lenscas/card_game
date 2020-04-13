use crate::{errors::ReturnErrors, util::CastRejection};
use rlua::{Lua, UserData, UserDataMethods};
use serde::{Deserialize, Serialize};
use sqlx::pool::PoolConnection;
use sqlx::{query, PgConnection};
use std::{error::Error, fmt::Display, fs::read_to_string as read_to_string_sync, sync::Arc};
use tokio::fs::read_to_string;

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Battle {
    pub(crate) player: Player,
    pub(crate) ai: Player,
}

impl Battle {
    pub(crate) async fn new(
        player_id: i64,
        con: &mut PoolConnection<PgConnection>,
    ) -> Result<Battle, ReturnErrors> {
        let v = query!(
            r#"
                SELECT cards.id,cards.json_file_path
                FROM cards
                INNER JOIN cards_in_deck
                ON cards_in_deck.card_id = cards.id
                INNER JOIN decks
                ON decks.id = cards_in_deck.deck_id
                INNER JOIN characters
                ON characters.id = decks.character_id
                WHERE characters.user_id = $1
            "#,
            player_id
        )
        .fetch_all(con)
        .await?;
        let mut cards = Vec::new();
        for card_id in v {
            let path = format!("./lua/compiled/cards/{}", card_id.json_file_path);
            cards.push(
                read_to_string(&path)
                    .await
                    .map_err(|v| {
                        println!("Error reading {}, error: {}", path, v);
                        v
                    })
                    .half_cast()
                    .and_then(|v| serde_json::from_str(&v).half_cast())?,
            );
        }
        let mut player = Player {
            hand: vec![],
            life: 20,
            deck: cards.iter().cloned().collect(),
            mana: 0,
            runes: Default::default(),
            rune_count: 0,
        };
        player.fill_hand();
        Ok(Battle {
            player: player.clone(),
            ai: player,
        })
    }
    pub async fn process_turn(
        self,
        chosen_card: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(card) = self.player.hand.get(chosen_card) {
            if card.cost > self.player.mana {
                panic!("Card cost can't be higher than available mana")
            }
        } else {
            panic!("Selected card does not exist")
        }
        let lua = Lua::new();
        let engine = read_to_string("./lua/engine.lua").await?;
        let mut battle =
            lua.context::<_, Result<_, Box<dyn std::error::Error>>>(move |lua_ctx| {
                let globals = lua_ctx.globals();
                globals.set("battle", self)?;
                globals.set("chosenCard", chosen_card)?;
                let v = lua_ctx.load(&engine).set_name("test?")?.eval::<Battle>()?;
                Ok(v)
            })?;
        battle.player.fill_hand();
        battle.ai.fill_hand();
        Ok(battle)
    }
}
impl UserData for Battle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_ai_card", |_, me, _: ()| {
            let index = 0;
            let item = me.ai.hand[index].clone();
            Ok((item, index))
        });
        methods.add_method("get_player_card", |_, me, index: usize| {
            let item = me.player.hand.get(index).map(|v| v.clone());
            Ok((item, index))
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
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct SmallRune {
    pub(crate) config: SmallRuneRaw,
    pub(crate) name: String,
    pub(crate) id: u64,
}

impl SmallRune {
    pub(crate) fn new(id: u64, config: SmallRuneRaw, name: String) -> Self {
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct SimpleError(pub String);
impl SimpleError {
    pub fn new(str: String) -> rlua::Error {
        rlua::Error::ExternalError(Arc::new(Self(str)))
    }
}
impl Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl Error for SimpleError {}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct SmallRuneRaw {
    pub(crate) turns_left: u64,
}

impl UserData for SmallRuneRaw {}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Player {
    pub(crate) hand: Vec<Card>,
    pub(crate) life: u64,
    pub(crate) deck: Vec<Card>,
    pub(crate) mana: u64,
    pub(crate) runes: [Option<SmallRune>; 5],
    pub(crate) rune_count: u64,
}

impl Player {
    pub(crate) fn fill_hand(&mut self) {
        let amount_needed = 7 - self.hand.len();
        let deck = &mut self.deck;
        let hand = &mut self.hand;
        (0..amount_needed)
            .filter_map(|_| {
                if deck.len() > 0 {
                    Some(deck.remove(0))
                } else {
                    None
                }
            })
            .for_each(|card| hand.push(card));
    }
}

impl UserData for Player {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method_mut("remove_card", |_, me, card| {
            println!("to remove : {}", card);
            if me.hand.len() > card {
                println!("has removed : {}", card);
                me.hand.remove(card);
            } else {
                return Err(SimpleError::new(format!("Index {} out of bounds", card)));
            }
            Ok(())
        });
        methods.add_method_mut("add_mana", |_, me, amount| {
            me.mana = me.mana.checked_add(amount).unwrap_or_else(u64::max_value);
            Ok(())
        });
        methods.add_method_mut("sub_mana", |_, me, amount| {
            match me.mana.checked_sub(amount) {
                Some(x) => {
                    me.mana = x;
                    Ok(true)
                }
                None => Ok(false),
            }
        });
        methods.add_method("get_mana", |_, me, _: ()| Ok(me.mana));
        methods.add_method_mut("deal_damage", |_, me, damage: u64| {
            me.life = me.life.checked_sub(damage).unwrap_or(0);
            Ok(())
        });
        methods.add_method_mut("heal", |_, me, heal_by: u64| {
            me.life = me.life.checked_add(heal_by).unwrap_or(u64::max_value());
            Ok(())
        });
        methods.add_method("get_runes", |_, me, _: ()| {
            Ok(me.runes.iter().cloned().collect::<Vec<_>>())
        });

        methods.add_method_mut("save_rune", |_, me, (rune, index): (SmallRune, usize)| {
            if let Some(old_rune) = me.runes.get_mut(index).and_then(|v| v.as_mut()) {
                *old_rune = rune;
            } else {
                return Err(SimpleError::new(format!(
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
                "./lua/compiled/small_runes/config/{}.lua",
                rune_name
            ))
            .map_err(|v| {
                dbg!(rune_name.clone());
                rlua::Error::ExternalError(Arc::new(v))
            })?;
            let rune = serde_json::from_str(&as_str)
                .map_err(|v| rlua::Error::ExternalError(Arc::new(v)))?;
            let rune = SmallRune::new(me.rune_count, rune, rune_name);
            for v in &mut me.runes {
                if let None = v {
                    me.rune_count += 1;
                    *v = Some(rune.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                let mut first: Option<(usize, SmallRune)> = None;
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

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Card {
    pub(crate) name: String,
    pub(crate) speed: u8,
    pub(crate) cost: u64,
    pub(crate) code: String,
}
impl UserData for Card {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_cost", |_, me, _: ()| Ok(me.cost));
        methods.add_method("get_speed", |_, me, _: ()| Ok(me.speed));
        methods.add_method("get_name", |_, me, _: ()| Ok(me.name.clone()));
        methods.add_method("get_code", |_, me, _: ()| Ok(me.code.clone()));
    }
}
