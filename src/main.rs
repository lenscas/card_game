use crate::screens::screen::Screen;
pub(crate) use client::Client;
use quicksilver::input::Event::{PointerMoved};
use quicksilver::{
    geom::{Vector},
    graphics::{Graphics, Image, ResizeHandler, VectorFont},
    input::Input,
    load_file, run,
    saving::Location,
    Settings, Window,
};
use std::error::Error as TError;

use mergui::{Context};

mod animations;
mod client;
mod responses;
mod screen_parts;
mod screens;

const SIZE: Vector = Vector { x: 1366., y: 768. };
const APP_NAME: &str = "Card game";

pub(crate) type Error = Box<dyn TError + 'static + Send + Sync>;
pub(crate) type Result<T> = std::result::Result<T, Error>;
fn main() {
    run(
        Settings {
            size: SIZE,
            title: "Card game",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

pub(crate) struct Wrapper {
    pub window: Window,
    pub gfx: Graphics,
    pub events: Input,
    pub context: Context,
    pub client: Client,
    pub font: VectorFont,
    pub button_image: Image,
    cursor_at: Vector,
}
impl Wrapper {
    pub(crate) fn cursor_at(&self) -> Vector {
        self.gfx.screen_to_camera(&self.window, self.cursor_at)
    }
}

async fn app(window: Window, gfx: Graphics, events: Input) -> Result<()> {
    let context = Context::new();

    let last_used_url = match quicksilver::saving::load::<String>(
        Location::Config,
        APP_NAME,
        "last_connected_server",
    ) {
        Ok(x) => x,
        Err(_) => String::from_utf8(load_file("default_server.txt").await?).unwrap(),
    };
    let font = VectorFont::load("font.ttf").await?;
    let button_image = Image::load(&gfx, "./button.png").await?;
    let mut wrapper = Wrapper {
        window,
        gfx,
        events,
        context,
        client: Client::new(last_used_url),
        cursor_at: Vector::new(0., 0.),
        font,
        button_image,
    };
    let mut v: Box<dyn Screen> = Box::new(screens::Login::new(&mut wrapper).await?);
    v.draw(&mut wrapper).await?;

    // Create a ResizeHandler that will Fit the content to the screen, leaving off area if we need
    // to. Here, we provide an aspect ratio of 4:3.
    let resize_handler = ResizeHandler::Fit {
        aspect_width: 16.0,
        aspect_height: 9.0,
    };
    wrapper.gfx.set_resize_handler(resize_handler);
    loop {
        while let Some(e) = wrapper.events.next_event().await {
            if let PointerMoved(e) = &e {
                wrapper.cursor_at = e.location();
            }
            wrapper.context.event(&e, &wrapper.window);
            if let Some(x) = v.event(&mut wrapper, &e).await? {
                v = x;
            }
        }
        if let Some(x) = v.update(&mut wrapper).await? {
            v = x;
        }
        v.draw(&mut wrapper).await?;
        wrapper.context.render(&mut wrapper.gfx, &wrapper.window)?;
        wrapper.gfx.present(&wrapper.window)?;
    }
}
