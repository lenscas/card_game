use super::{Card, SimpleError, SmallRune};
use rlua::{UserData, UserDataMethods};
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string as read_to_string_sync, sync::Arc};

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
