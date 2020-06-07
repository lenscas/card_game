use super::{Battle, Screen};
use crate::{Result as CResult, Wrapper};
use async_trait::async_trait;
use mergui::{
    channels::{BasicClickable, InputChannel},
    core::Text,
    widgets::{
        input::{InputConfig, PlaceholderConfig},
        ButtonConfig,
    },
    FontStyle, LayerId, MFont, Response,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image, VectorFont},
};

pub(crate) struct Login {
    _layer: LayerId,
    _text: Response<()>,
    name_input: Response<InputChannel>,
    password_input: Response<InputChannel>,
    login_button: Response<BasicClickable>,
}

impl Login {
    pub(crate) async fn new(wrapper: &mut Wrapper) -> CResult<Self> {
        let layer = wrapper.context.add_layer();
        let ttf = VectorFont::load("font.ttf").await?;
        let font = MFont::from_font(&ttf, &wrapper.gfx, 30.0)?;
        let basic_font_style = FontStyle {
            font: font.clone(),
            location: Vector::new(350., 40.),
            color: Color::BLACK,
        };
        let conf = Text {
            text: "Login".into(),
            font_style: basic_font_style.clone(),
        };
        let _text = wrapper.context.add_widget(conf, &layer).unwrap();

        let input_font = FontStyle {
            font: MFont::from_font(&ttf, &wrapper.gfx, 20.0)?,
            ..basic_font_style.clone()
        };
        let placeholder_font = FontStyle {
            color: Color::from_hex("#746868"),
            ..input_font.clone()
        };
        let conf = InputConfig {
            font: input_font.clone(),
            placeholder: Some(PlaceholderConfig {
                font: placeholder_font.clone(),
                text: "Username".into(),
            }), //Option<PlaceholderConfig>,
            location: Rectangle::new(Vector::new(200., 200.), Vector::new(300., 25.)),
            start_value: None,
            cursor_config: Default::default(),
        };
        let name_input = wrapper.context.add_widget(conf, &layer).unwrap();

        let conf = InputConfig {
            font: input_font.clone(),
            placeholder: Some(PlaceholderConfig {
                font: placeholder_font,
                text: "Password".into(),
            }), //Option<PlaceholderConfig>,
            location: Rectangle::new(Vector::new(200., 230.), Vector::new(300., 25.)),
            start_value: None,
            cursor_config: Default::default(),
        };
        let password_input = wrapper.context.add_widget(conf, &layer).unwrap();

        let conf = ButtonConfig {
            text: "Login".into(),
            font_style: FontStyle {
                color: Color::WHITE,
                location: Vector::new(10., 20.),
                ..input_font
            },
            background: Image::load(&wrapper.gfx, "button.png").await?,
            background_location: Rectangle::new(Vector::new(510., 230.), Vector::new(70., 30.)),
            blend_color: Some(Color::from_hex("#008B24")),
            hover_color: Some(Color::from_hex("#07C739")),
        };
        let login_button = wrapper.context.add_widget(conf, &layer).unwrap();

        Ok(Login {
            _layer: layer,
            _text,
            name_input,
            password_input,
            login_button,
        })
    }
}

#[async_trait(?Send)]
impl Screen for Login {
    async fn draw(&mut self, wrapper: &mut Wrapper) -> CResult<()> {
        wrapper.gfx.clear(Color::WHITE);
        Ok(())
    }
    async fn update(&mut self, _: &mut Wrapper) -> CResult<Option<Box<dyn Screen>>> {
        Ok(None)
    }
    async fn event(
        &mut self,
        wrapper: &mut Wrapper,
        _: &quicksilver::input::Event,
    ) -> CResult<Option<Box<dyn Screen>>> {
        if self.login_button.channel.has_clicked()
            && self.password_input.channel.get() != ""
            && self.name_input.channel.get() != ""
        {
            wrapper
                .client
                .log_in(
                    self.name_input.channel.get(),
                    self.password_input.channel.get(),
                )
                .await?;
            return Ok(Some(Box::new(Battle::new(wrapper).await?)));
        }
        Ok(None)
    }
}
