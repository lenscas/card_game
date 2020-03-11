use crate::{
    responses::{CustomResult, ErrorRes, LoginResponse},
    Wrapper,
};
use mergui::{
    channels::Clickable,
    core::Text,
    widgets::{
        input::{InputConfig, PlaceholderConfig},
        ButtonConfig,
    },
    FontStyle,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
    Result as QResult,
};
use reqwest::blocking as reqw;

pub(crate) async fn login<'a>(mut wrapper: Wrapper<'_>) -> QResult<Wrapper<'_>> {
    let layer = wrapper.context.add_layer();
    let basic_font_style = FontStyle {
        font: wrapper.basic_font.clone(),
        size: 30.0,
        location: Vector::new(350, 40),
        color: Color::BLACK,
        max_width: None,
    };
    let conf = Text {
        text: "Login".into(),
        font_style: basic_font_style.clone(),
    };
    let _t = wrapper.context.add_widget(conf, &layer).unwrap();

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
    let mut login_button = wrapper.context.add_widget(conf, &layer).unwrap();
    loop {
        while let Some(e) = wrapper.events.next_event().await {
            wrapper.context.event(&e, &wrapper.window);
            if login_button.channel.has_clicked()
                && password_input.channel.get() != ""
                && name_input.channel.get() != ""
            {
                let client = reqw::Client::new();
                let x = client
                    .post("http://127.0.0.1:3030/login")
                    .body(format!(
                        "{{\"username\":\"{}\",\"password\":\"{}\"}}",
                        name_input.channel.get(),
                        password_input.channel.get()
                    ))
                    .send()
                    .expect("Something wend wrong :(");
                let res: CustomResult<LoginResponse> =
                    serde_json::from_str(&x.text().unwrap()).unwrap();
                let v = Result::<_, _>::from(res).unwrap();
                println!("{:?}", v);
            }
        }
        wrapper.gfx.clear(Color::WHITE);
        wrapper.context.render(&mut wrapper.gfx);
        wrapper.gfx.present(&wrapper.window)?;
    }
    //Ok(wrapper)
}
