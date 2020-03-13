use crate::screens::screen::Screen;
use quicksilver::{
    geom::Vector,
    graphics::Graphics,
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

use mergui::Context;

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
}

async fn app(window: Window, gfx: Graphics, events: EventStream) -> Result<()> {
    let context = Context::new([0.0, 0.0].into());
    let mut wrapper = Wrapper {
        window,
        gfx,
        events,
        context,
    };
    let mut v: Box<dyn Screen> = Box::new(screens::Login::new(&mut wrapper).await?);
    v.draw(&mut wrapper).await?;
    loop {
        while let Some(e) = wrapper.events.next_event().await {
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
