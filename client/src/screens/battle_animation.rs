use super::{Screen, Battle};
use async_trait::async_trait;
use card_game_shared::battle_log::ActionsDuringTurn;
pub(crate) struct BattleAnimation {
    battle : Battle,
    _events : ActionsDuringTurn
}
impl BattleAnimation {
    pub fn new(battle: Battle, events : ActionsDuringTurn) -> Self {
        Self {
            battle,_events : dbg!(events)
        }
    }
}

#[async_trait(?Send)]
impl Screen for BattleAnimation {
    async fn draw(&mut self, wrapper: &mut crate::Wrapper) -> crate::Result<()> {
        self.battle.draw(wrapper).await
    }

    async fn update(mut self : Box<Self>, wrapper: &mut crate::Wrapper) -> crate::Result<Box<dyn Screen>> {
        self.battle.update(wrapper).await?;
        Ok(Box::new(self.battle))
    }

    async fn event(
        self : Box<Self>,
        _wrapper: &mut crate::Wrapper,
        _event: &quicksilver::input::Event,
    ) -> crate::Result<Box<dyn Screen>> {
        Ok(self)
    }
}