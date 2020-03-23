use super::Screen;
use async_trait::async_trait;
use quicksilver::geom::{Circle, Rectangle, Shape, Vector};
use quicksilver::graphics::{Color, Font};
use quicksilver::mint::Vector2;
use std::f64::consts::PI;

use crate::Wrapper;

pub struct Battle {
    outer_points: Vec<Circle>,
    inner_points: Vec<Circle>,
    hand: Vec<(String, Rectangle)>,
    rotation: f64,
    return_is_down: bool,
    font: Font,
    clicked: bool,
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

fn get_location_of_cards(cards: Vec<String>, resolution: Vector2<f32>) -> Vec<(String, Rectangle)> {
    cards
        .into_iter()
        .enumerate()
        .map(|(key, card)| {
            let rec_size = (0.1375 * resolution.x, 0.283333333333 * resolution.y);
            let rec_location = (
                0.00625 * resolution.x,
                (0.00833333333333 + (0.0466666666667 * key as f32)) * resolution.y,
            );
            (card, Rectangle::new(rec_location, rec_size))
        })
        .collect()
}

impl Battle {
    pub(crate) async fn new(
        wrapper: &mut Wrapper<'_>,
    ) -> Result<Battle, Box<dyn std::error::Error>> {
        let v = wrapper.window.size();
        let outer_radius = 0.4f64 * f64::from(v.y);
        let outer_points = calc_points(outer_radius, 8, 10.0, |x: f64, y: f64, _| {
            (
                x + f64::from(0.500625 * v.x),
                y + f64::from(0.500833333333 * v.y), /*300.5f64*/
            )
        });
        let inner_radius = 0.233333333333 * f64::from(v.y);
        let inner_points = calc_points(inner_radius, 5, 0.0, |x, y, _| {
            (
                x + f64::from(0.500625 * v.x),
                y + f64::from(0.500833333333 * v.y),
            )
        });
        let current = wrapper.client.new_battle().await?;
        let resolution = v;
        let hand = get_location_of_cards(current.hand, resolution);

        Ok(Battle {
            outer_points,
            inner_points,
            rotation: 0.0,
            hand: hand,
            return_is_down: false,
            font: Font::load_ttf(&wrapper.gfx, "font.ttf").await.unwrap(),
            clicked: false,
        })
    }
    async fn play_card(&mut self, wrapper: &Wrapper<'_>) -> Result<(), Box<dyn std::error::Error>> {
        let cursor_pos = wrapper.get_cursor_loc();
        let chosen = self
            .hand
            .iter()
            .enumerate()
            .rev()
            .map(|(key, card)| (key, card.1))
            .find(|(_, card)| card.contains(cursor_pos))
            .map(|(k, _)| k);
        if let Some(chosen) = chosen {
            let battle = wrapper.client.do_turn(chosen).await?;
            self.hand = get_location_of_cards(battle.hand, wrapper.window.size());
        }
        Ok(())
    }
}

#[async_trait(?Send)]
impl Screen for Battle {
    async fn draw(&mut self, wrapper: &mut crate::Wrapper<'_>) -> quicksilver::Result<()> {
        let resolution = wrapper.window.size();
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
        wrapper.gfx.fill_circle(
            &Circle::new((resolution.x / 2f32, resolution.y / 2f32), 20),
            Color::WHITE,
        );
        wrapper
            .gfx
            .stroke_path(&[(0, 0).into(), resolution.into()], Color::BLUE);
        let mut font = &mut self.font;
        self.hand.iter_mut().for_each(|(card, rectangle)| {
            let mut card = card;
            let font_size = 0.0233333333333 * resolution.y;
            let font_location = Vector::new(
                (0.0025 * resolution.x) + rectangle.pos.x,
                (0.0025 * resolution.x) + font_size + rectangle.pos.y,
            );
            let rec = Rectangle::new(rectangle.pos, rectangle.size);
            wrapper.gfx.fill_rect(&rec, Color::WHITE);
            wrapper.gfx.stroke_rect(&rec, Color::RED);
            wrapper.gfx.draw_text(
                &mut font,
                &mut card,
                font_size,
                Some(165.0),
                Color::BLACK,
                font_location,
            );
            //wrapper.gfx.draw_text(font, text, size, max_width, color, offset)
            //println!("{},{}", key, card)
        });
        Ok(())
    }
    async fn update(
        &mut self,
        wrapper: &mut crate::Wrapper<'_>,
    ) -> quicksilver::Result<Option<Box<dyn Screen>>> {
        let v = wrapper.window.size();
        self.rotation += 0.0005;
        let inner_radius = 0.233333333333 * f64::from(v.y);
        self.inner_points = calc_points(inner_radius, 5, self.rotation, |x, y, _| {
            (
                x + f64::from(0.500625 * v.x),
                y + f64::from(0.500833333333 * v.y),
            )
        });
        Ok(None)
    }
    async fn event(
        &mut self,
        wrapper: &mut Wrapper<'_>,
        event: &quicksilver::lifecycle::Event,
    ) -> quicksilver::Result<Option<Box<dyn Screen>>> {
        use quicksilver::lifecycle::{Event::*, Key, MouseButton};
        match event {
            KeyboardInput(x) => {
                if x.key() == Key::Return {
                    if x.is_down() && self.return_is_down {
                        return Ok(None);
                    } else if x.is_down() && !self.return_is_down {
                        self.hand = get_location_of_cards(
                            wrapper.client.do_turn(0).await.unwrap().hand,
                            wrapper.window.size(),
                        );
                        self.return_is_down = true;
                    } else if !x.is_down() {
                        self.return_is_down = false;
                    }
                }
            }
            PointerInput(x) if x.button() == MouseButton::Left => {
                if x.is_down() {
                    if !self.clicked {
                        self.clicked = true;
                        self.play_card(wrapper).await.unwrap();
                    }
                } else {
                    self.clicked = false;
                }
            }
            _ => {}
        }
        Ok(None)
    }
}
