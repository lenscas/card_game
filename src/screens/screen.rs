use crate::{Result, Wrapper};
use async_trait::async_trait;
use quicksilver::input::Event;

#[async_trait(?Send)]
pub(crate) trait Screen {
    async fn draw(&mut self, wrapper: &mut Wrapper) -> Result<()>;
    async fn update(&mut self, wrapper: &mut Wrapper) -> Result<Option<Box<dyn Screen>>>;
    async fn event(
        &mut self,
        _wrapper: &mut Wrapper,
        _event: &Event,
    ) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
