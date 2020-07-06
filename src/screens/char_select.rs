use super::Screen;
use crate::{Result, Wrapper};
use async_trait::async_trait;
use mergui::{channels::BasicClickable, widgets::ButtonConfig, FontStyle, MFont, Response};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
};

enum ButtonType {
    Old,
    New,
}
pub(crate) struct CharacterSelect {
    button: (ButtonType, Response<BasicClickable>),
}

impl CharacterSelect {
    pub(crate) async fn new(wrapper: &mut Wrapper) -> Result<Self> {
        let characters = wrapper.client.get_characters().await?;
        let mut layer = wrapper.context.add_layer();

        let v = characters
            .characters
            .get(0)
            .map(|v| {
                Result::<_>::Ok((
                    ButtonType::Old,
                    layer.add_widget(ButtonConfig {
                        text: "Current character".into(),
                        font_style: FontStyle {
                            font: MFont::from_font(&wrapper.font, &wrapper.gfx, 20.0)?,
                            location: Vector::new(70., 25.),
                            color: Color::WHITE,
                        },
                        background: wrapper.button_image.clone(),
                        background_location: Rectangle::new(
                            Vector::new(653., 304.),
                            Vector::new(300., 40.),
                        ),
                        blend_color: Some(Color::GREEN),
                        hover_color: Some(Color::CYAN),
                    }),
                ))
            })
            .unwrap_or_else(|| {
                Ok((
                    ButtonType::New,
                    layer.add_widget(ButtonConfig {
                        text: "New character".into(),
                        font_style: FontStyle {
                            font: MFont::from_font(&wrapper.font, &wrapper.gfx, 20.0)?,
                            location: Vector::new(10., 20.),
                            color: Color::WHITE,
                        },
                        background: wrapper.button_image.clone(),
                        background_location: Rectangle::new(
                            Vector::new(653., 304.),
                            Vector::new(100., 30.),
                        ),
                        blend_color: Some(Color::GREEN),
                        hover_color: Some(Color::CYAN),
                    }),
                ))
            })?;
        //layer.add_widget(widget_config)
        Ok(Self { button: v })
    }
}

#[async_trait(?Send)]
impl Screen for CharacterSelect {
    async fn draw(&mut self, wrapper: &mut Wrapper) -> Result<()> {
        wrapper.gfx.clear(Color::ORANGE);
        Ok(())
    }
    async fn update(&mut self, wrapper: &mut Wrapper) -> crate::Result<Option<Box<dyn Screen>>> {
        if self.button.1.channel.has_clicked() {
            //code here
        }
        Ok(None)
    }
}
