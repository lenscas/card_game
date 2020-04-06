use crate::screens::screen::Screen;
use quicksilver::lifecycle::Event::PointerMoved;
use quicksilver::{
    geom::Vector,
    graphics::Graphics,
    lifecycle::{run, EventStream, Settings, Window},
    mint::Vector2,
    Result,
};

use mergui::Context;

mod client;
mod responses;
mod screens;
pub(crate) use client::Client;

fn main() {
    run(
        Settings {
            size: Vector::new(1366, 768).into(),
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
    pub client: Client,
    pub cursor_at: Vector2<f32>,
}
impl<'a> Wrapper<'a> {
    pub(crate) fn get_cursor_loc(&self) -> Vector2<f32> {
        self.cursor_at
    }
    pub(crate) fn get_pos_vector(&self, x: f32, y: f32) -> Vector {
        let res = self.window.size();
        Vector::new(x * res.x, y * res.y)
    }
}

async fn app(window: Window, gfx: Graphics, events: EventStream) -> Result<()> {
    let context = Context::new([0.0, 0.0].into());
    let mut wrapper = Wrapper {
        window,
        gfx,
        events,
        context,
        client: Client::new("http://127.0.0.1:3030/".into()),
        cursor_at: Vector2::from_slice(&[0f32, 0f32]),
    };
    let mut v: Box<dyn Screen> = Box::new(screens::Login::new(&mut wrapper).await?);
    v.draw(&mut wrapper).await?;
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
        wrapper.context.render(&mut wrapper.gfx);
        wrapper.gfx.present(&wrapper.window)?;
    }
}
