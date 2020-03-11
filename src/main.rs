/*
use reqwest::blocking as reqw;
use std::error::Error;
use std::io::stdin;

fn main() -> Result<(), Box<dyn Error>> {
    let input = stdin();
    let mut buffer = String::new();

    println!("username");
    input.read_line(&mut buffer)?;
    let username = buffer.clone();
    println!("password");
    input.read_line(&mut buffer)?;
    let password = buffer.clone();
    let client = reqw::Client::new();
    let x = client
        .post("http://127.0.0.1:3030/login")
        .body(format!(
            "{{\"username\":\"{}\",\"password\":\"{}\"",
            username, password
        ))
        .send()?;
    println!("{}", x.text()?);
    Ok(())
    //loop {}
}
*/
use mergui::widgets::{input::InputConfig, ButtonConfig, ConcealerConfig, DropDownConfig};
use quicksilver::graphics::blend::{
    BlendChannel, BlendFactor, BlendFunction, BlendInput, BlendMode,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

use mergui::{core::Text, Context, FontStyle, MFont};
use std::marker::PhantomData;

mod responses;
mod screens;

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Image Example",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

pub(crate) struct Wrapper<'a> {
    pub window: Window,
    pub gfx: Graphics,
    pub events: EventStream,
    pub context: Context<'a>,
    pub basic_font: MFont,
}

async fn app(window: Window, gfx: Graphics, events: EventStream) -> Result<()> {
    // Load the Font, just like loading any other asset
    let font = MFont::load_ttf(&gfx, "font.ttf").await?;
    let context = Context::new([0.0, 0.0].into());
    let wrapper = Wrapper {
        window,
        gfx,
        events,
        context,
        basic_font: font,
    };
    screens::login(wrapper).await.map(drop)
    /*
    let layer = context.add_layer();

    let basic_font_style = FontStyle {
        font: font.clone(),
        size: 15.0,
        location: Vector::new(20, 20),
        color: Color::BLACK,
        max_width: None,
    };

    let conf = Text {
        text: "Some awesome piece of text".into(),
        font_style: basic_font_style.clone(),
    };
    let _t = context.add_widget(conf, &layer).unwrap();
    let button = Image::load(&gfx, "button.png").await?;
    gfx.set_blend_mode(Some(BlendMode {
        equation: Default::default(),
        function: BlendFunction::Same {
            source: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: false,
            },
            destination: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: true,
            },
        },
        global_color: [0.0; 4],
    }));

    let conf = ButtonConfig {
        background: button.clone(),
        background_location: Rectangle::new((100, 50), (200, 100)),
        blend_color: Some(Color::GREEN),
        hover_color: Some(Color::RED),
        font_style: FontStyle {
            font: font.clone(),
            size: 20.0,
            location: Vector::new(30, 55),
            color: Color::BLUE,
            max_width: None,
        },
        text: "Some text".into(),
    };
    let _button = context.add_widget(conf, &layer).unwrap();
    let conf = ConcealerConfig {
        button: ButtonConfig {
            background: button.clone(),
            background_location: Rectangle::new((100, 155), (200, 100)),
            blend_color: Some(Color::GREEN),
            hover_color: Some(Color::RED),
            font_style: FontStyle {
                font: font.clone(),
                size: 20.0,
                location: Vector::new(30, 55),
                color: Color::BLUE,
                max_width: None,
            },
            text: "Concealer".into(),
        },
        hidden_widgets: vec![(
            0,
            ButtonConfig {
                background: button.clone(),
                background_location: Rectangle::new((310, 155), (200, 100)),
                blend_color: Some(Color::GREEN),
                hover_color: Some(Color::RED),
                font_style: FontStyle {
                    font: font.clone(),
                    size: 20.0,
                    location: Vector::new(30, 55),
                    color: Color::BLUE,
                    max_width: None,
                },
                text: "Hidden".into(),
            },
        )],
        to_widget: PhantomData,
        to_result: PhantomData,
    };
    let _concealer = context.add_widget(conf, &layer).unwrap();

    let conf = DropDownConfig {
        values: vec![
            (
                "awesome",
                FontStyle {
                    size: 30.0,
                    location: Vector::new(10, 55),
                    ..basic_font_style.clone()
                },
            ),
            (
                "second",
                FontStyle {
                    size: 35.0,
                    location: Vector::new(15, 55),
                    ..basic_font_style.clone()
                },
            ),
        ],
        location: Rectangle::new((100, 300), (160, 50)),
        option_height: 50.0,
        open_button: button.clone(),
        open_button_size: [100.0, 50.0].into(),
        selected: Some(0),
        divider_color: Color::BLACK,
        divider_size: 5.0,
        t: PhantomData,
    };
    let _dropdown = context.add_widget(conf, &layer).unwrap();

    let config = InputConfig {
        font: FontStyle {
            size: 45.0,
            ..basic_font_style.clone()
        },
        placeholder: None, //Option<PlaceholderConfig>,
        location: Rectangle::new((100, 355), (160, 50)),
        start_value: None,
    };
    let _text_input = context.add_widget(config, &layer).unwrap();
    gfx.clear(Color::WHITE);
    context.render(&mut gfx);

    gfx.present(&window)?;

    loop {
        while let Some(e) = events.next_event().await {
            context.event(&e, &window);
        }
        gfx.clear(Color::WHITE);
        context.render(&mut gfx);
        gfx.present(&window)?;
    }*/
}
