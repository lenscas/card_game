use crate::{Result, Wrapper};
use async_trait::async_trait;
use quicksilver::input::Event;
use std::{future::Future, pin::Pin};

#[async_trait(?Send)]
pub(crate) trait Screen {
    async fn draw(&mut self, wrapper: &mut Wrapper) -> Result<()>;
    async fn update(mut self : Box<Self>, wrapper: &mut Wrapper) -> Result<Box<dyn Screen>>;
    async fn event(
        self : Box<Self>,
        _wrapper: &mut Wrapper,
        _event: &Event,
    ) -> Result<Box<dyn Screen>>;
}
