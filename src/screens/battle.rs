use super::Screen;
use async_trait::async_trait;
use quicksilver::geom::Circle;
use quicksilver::graphics::Color;
use std::f64::consts::PI;

pub struct Battle {
    outer_points: Vec<Circle>,
    inner_points: Vec<Circle>,
    rotation: f64,
}

fn calc_points(
    radius: f64,
    points: i8,
    rotation: f64,
    offset: impl Fn(f64, f64, f64) -> (f64, f64),
) -> Vec<Circle> {
    let radius: f64 = radius;
    let steps: f64 = 2.0 * PI / f64::from(points); //0.78539816339;
                                                   //let steps: f64 = 1.0471975512;
    (0i8..points)
        .into_iter()
        .map(|v| f64::from(v) * steps + rotation)
        .map(|v| (radius * (v.sin()), radius * (v.cos())))
        .map(|(x, y)| offset(x, y, radius))
        .map(|(x, y)| Circle::new((x as f32, y as f32), 35))
        .collect()
}

impl Battle {
    pub(crate) fn new() -> Battle {
        let outer_points = calc_points(250f64, 8, 10.0, |x, y, _| (x + 400.5, y + 300.5));
        let inner_points = calc_points(140f64, 5, 0.0, |x, y, _| (x + 400.5, y + 300.5));
        Battle {
            outer_points,
            inner_points,
            rotation: 0.0,
        }
    }
}

#[async_trait(?Send)]
impl Screen for Battle {
    async fn draw(&mut self, wrapper: &mut crate::Wrapper<'_>) -> quicksilver::Result<()> {
        wrapper.gfx.clear(Color::BLACK);
        self.outer_points
            .iter()
            .enumerate()
            .for_each(|(key, circle)| {
                wrapper
                    .gfx
                    .fill_circle(circle, Color::from_rgba(255, 0, key as u8 * 31, 1.0));
                wrapper.gfx.draw_point(circle.pos, Color::WHITE);
            });
        self.inner_points
            .iter()
            .enumerate()
            .for_each(|(key, circle)| {
                wrapper
                    .gfx
                    .fill_circle(circle, Color::from_rgba(0, 255, key as u8 * 63, 1.0));
                wrapper.gfx.draw_point(circle.pos, Color::WHITE);
            });
        wrapper
            .gfx
            .fill_circle(&Circle::new((400.5, 300), 20), Color::WHITE);
        wrapper
            .gfx
            .stroke_path(&[(0, 0).into(), (800, 600).into()], Color::BLUE);
        Ok(())
    }
    async fn update(
        &mut self,
        _: &mut crate::Wrapper<'_>,
    ) -> quicksilver::Result<Option<Box<dyn Screen>>> {
        self.rotation += 0.0005;
        self.inner_points = calc_points(140f64, 5, self.rotation, |x, y, _| (x + 400.5, y + 300.5));
        Ok(None)
    }
}
