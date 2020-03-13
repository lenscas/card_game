use super::Screen;
use crate::{
    responses::{CustomResult, LoginResponse},
    Wrapper,
};
use async_trait::async_trait;
use mergui::{
    channels::{BasicClickable, Clickable, InputChannel},
    core::Text,
    widgets::{
        input::{InputConfig, PlaceholderConfig},
        ButtonConfig,
    },
    FontStyle, LayerId, MFont, Response,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    Result as QResult,
};
use reqwest::blocking as reqw;

pub(crate) struct Login {
    _layer: LayerId,
    _text: Response<()>,
    name_input: Response<InputChannel>,
    password_input: Response<InputChannel>,
    login_button: Response<BasicClickable>,
}

impl Login {
    pub(crate) async fn new<'a>(wrapper: &mut Wrapper<'a>) -> QResult<Self> {
        let layer = wrapper.context.add_layer();
        let font = MFont::load_ttf(&wrapper.gfx, "font.ttf").await?;
        let basic_font_style = FontStyle {
            font: font.clone(),
            size: 30.0,
            location: Vector::new(350, 40),
            color: Color::BLACK,
            max_width: None,
        };
        let conf = Text {
            text: "Login".into(),
            font_style: basic_font_style.clone(),
        };
        let _text = wrapper.context.add_widget(conf, &layer).unwrap();

        let input_font = FontStyle {
            size: 20.0,
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
            location: Rectangle::new((200, 200), (300, 25)),
            start_value: None,
        };
        let name_input = wrapper.context.add_widget(conf, &layer).unwrap();

        let conf = InputConfig {
            font: input_font.clone(),
            placeholder: Some(PlaceholderConfig {
                font: placeholder_font,
                text: "Password".into(),
            }), //Option<PlaceholderConfig>,
            location: Rectangle::new((200, 230), (300, 25)),
            start_value: None,
        };
        let password_input = wrapper.context.add_widget(conf, &layer).unwrap();

        let conf = ButtonConfig {
            text: "Login".into(),
            font_style: FontStyle {
                color: Color::WHITE,
                location: Vector::new(10, 20),
                ..input_font
            },
            background: Image::load(&wrapper.gfx, "button.png").await?,
            background_location: Rectangle::new((510, 230), (70, 30)),
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
    async fn draw(&mut self, wrapper: &mut Wrapper<'_>) -> QResult<()> {
        wrapper.gfx.clear(Color::WHITE);
        Ok(())
    }
    async fn update(&mut self, _: &mut Wrapper<'_>) -> QResult<Option<Box<dyn Screen>>> {
        Ok(None)
    }
    async fn event(
        &mut self,
        wrapper: &mut Wrapper<'_>,
        e: &quicksilver::lifecycle::Event,
    ) -> QResult<Option<Box<dyn Screen>>> {
        wrapper.context.event(&e, &wrapper.window);
        if self.login_button.channel.has_clicked()
            && self.password_input.channel.get() != ""
            && self.name_input.channel.get() != ""
        {
            let client = reqw::Client::new();
            let x = client
                .post("http://127.0.0.1:3030/login")
                .body(format!(
                    "{{\"username\":\"{}\",\"password\":\"{}\"}}",
                    self.name_input.channel.get(),
                    self.password_input.channel.get()
                ))
                .send()
                .expect("Something wend wrong :(");
            let res: CustomResult<LoginResponse> =
                serde_json::from_str(&x.text().unwrap()).unwrap();
            let v = Result::<_, _>::from(res).unwrap();
            println!("{:?}", v);
        }
        Ok(None)
    }
}
