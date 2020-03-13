use crate::Wrapper;
use async_trait::async_trait;
use quicksilver::{lifecycle::Event, Result};

#[async_trait(?Send)]
pub(crate) trait Screen {
    async fn draw(&mut self, wrapper: &mut Wrapper<'_>) -> Result<()>;
    async fn update(&mut self, wrapper: &mut Wrapper<'_>) -> Result<Option<Box<dyn Screen>>>;
    async fn event(
        &mut self,
        _wrapper: &mut Wrapper<'_>,
        _event: &Event,
    ) -> Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
}
