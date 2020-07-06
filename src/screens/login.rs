use super::{Battle, CharacterSelect, Screen};
use crate::{Result as CResult, Wrapper, APP_NAME};
use async_trait::async_trait;
use mergui::{
    channels::{BasicClickable, ConcealerReturn, InputChannel},
    core::Text,
    widgets::{
        input::{InputConfig, PlaceholderConfig},
        ButtonConfig, ConcealerConfig,
    },
    FontStyle, MFont, Response,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image, VectorFont},
    saving::{save, Location},
};

pub(crate) struct Login {
    _text: Response<()>,
    name_input: Response<InputChannel>,
    password_input: Response<InputChannel>,
    login_button: Response<BasicClickable>,
    _concealer: Response<ConcealerReturn>,
    server_address: Response<InputChannel>,
}

impl Login {
    pub(crate) async fn new(wrapper: &mut Wrapper) -> CResult<Self> {
        let mut layer = wrapper.context.add_layer();
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

        let button_background = Image::load(&wrapper.gfx, "button.png").await?;

        let conf = ButtonConfig {
            text: "Login".into(),
            font_style: FontStyle {
                color: Color::WHITE,
                location: Vector::new(10., 20.),
                ..input_font.clone()
            },
            background: button_background.clone(),
            background_location: Rectangle::new(Vector::new(430., 260.), Vector::new(70., 30.)),
            blend_color: Some(Color::from_hex("#008B24")),
            hover_color: Some(Color::from_hex("#07C739")),
        };
        let login_button = wrapper.context.add_widget(conf, &layer).unwrap();

        let mut secret_layer = wrapper.context.add_singular_layer();

        let server_address = InputConfig {
            font: input_font.clone(),
            placeholder: None,
            location: Rectangle::new(Vector::new(200., 295.), Vector::new(300., 25.)),
            start_value: Some(wrapper.client.base_url.clone()),
            cursor_config: Default::default(),
        };
        let server_address = secret_layer.add_widget(server_address);

        let concealer_config = ConcealerConfig {
            button: ButtonConfig {
                text: "Advanced".into(),
                font_style: FontStyle {
                    location: Vector::new(10., 20.),
                    color: Color::WHITE,
                    ..input_font
                },
                background: button_background,
                background_location: (Rectangle::new(
                    Vector::new(200., 260.),
                    Vector::new(110., 30.),
                )),
                blend_color: Some(Color::from_hex("008B24")),
                hover_color: Some(Color::from_hex("#07C739")),
            },
            layer: secret_layer,
        };
        let concealer = layer.add_widget(concealer_config);

        Ok(Login {
            _text,
            name_input,
            password_input,
            login_button,
            _concealer: concealer,
            server_address,
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
            && self.server_address.channel.get() != ""
        {
            let new_address = self.server_address.channel.get();
            if new_address != wrapper.client.base_url {
                save(
                    Location::Config,
                    APP_NAME,
                    "last_connected_server",
                    &new_address,
                )?;
                wrapper.client.base_url = new_address;
            }
            return match wrapper
                .client
                .log_in(
                    self.name_input.channel.get(),
                    self.password_input.channel.get(),
                )
                .await
            {
                Ok(_) => Ok(Some(Box::new(CharacterSelect::new(wrapper).await?))),
                Err(_) => Ok(None),
            };
        }
        Ok(None)
    }
}
