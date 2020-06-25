use super::Card;
use crate::{errors::ReturnErrors, util::CastRejection};
use card_game_shared::battle::BattleErrors;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, query, PgConnection};
use tokio::fs::read_to_string;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Deck {
    pub(crate) hand: Vec<Card>,
    pub(crate) deck: Vec<Card>,
    pub(crate) casted: Vec<Card>,
}
impl Deck {
    pub(crate) async fn create_deck_for_player(
        player_id: i64,
        con: &mut PoolConnection<PgConnection>,
    ) -> Result<Self, ReturnErrors> {
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
        let mut deck = Self {
            hand: Vec::new(),
            deck: cards,
            casted: Vec::new(),
        };
        deck.fill_hand();
        Ok(deck)
    }
    pub(crate) fn get_card_from_hand(&mut self, index: usize) -> Result<Card, BattleErrors> {
        if self.hand.len() > index {
            let card = self.hand.remove(index);
            return if card.should_reshuffle {
                self.casted.push(card);
                Ok(self
                    .casted
                    .last()
                    .expect("Vec did not contain a last after push. Something broke BIG TIME")
                    .clone())
            } else {
                Ok(card)
            };
        }
        Err(BattleErrors::ChosenCardNotInHand(index))
    }
    pub fn get_ids_from_hand(&self) -> Vec<String> {
        self.hand.iter().map(|v| v.id.clone()).collect()
    }
    pub fn discard_card(&mut self) -> bool {
        match self.hand.pop() {
            Some(x) => {
                if x.should_reshuffle {
                    self.casted.push(x);
                }
                true
            }
            None => false,
        }
    }
    pub(crate) fn fill_hand(&mut self) {
        let amount_needed = 7 - self.hand.len();
        let deck = &mut self.deck;
        let hand = &mut self.hand;
        let casted = &mut self.casted;
        (0..amount_needed)
            .map(|_| {
                if deck.is_empty() && casted.is_empty() {
                    return None;
                }
                if deck.is_empty() {
                    std::mem::swap(deck, casted);
                }
                deck.pop()
            })
            .take_while(|v| v.is_some())
            .filter_map(|v| v)
            .for_each(|card| hand.push(card));
    }
}