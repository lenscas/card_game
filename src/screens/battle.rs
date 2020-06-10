use super::Screen;
use async_trait::async_trait;
use quicksilver::geom::{Circle, Rectangle, Shape, Vector};
use quicksilver::graphics::{Color, FontRenderer, Image, VectorFont};
use std::f64::consts::PI;

use crate::{
    animations::{calc_points, RuneAnimation},
    Wrapper, SIZE,
};

fn has_rune<'a>(
    index: usize,
    player_runes: &'a [String],
    enemy_runes: &'a [String],
) -> Option<&'a String> {
    if index > 0 && index <= 4 {
        player_runes.get(index - 1)
    } else if index == 0 {
        enemy_runes.get(index)
    } else {
        enemy_runes.get(8 - index)
    }
}

pub struct Battle {
    outer_points: Vec<Circle>,
    hexa_runes: RuneAnimation,
    hand: Vec<(Image, Rectangle)>,
    player_runes: Vec<String>,
    enemy_runes: Vec<String>,
    stat_font: FontRenderer,
    clicked: bool,
    enemy_hp: String,
    enemy_hand_size: String,
    player_hp: String,
    enemy_mana: String,
    player_mana: String,
    hover_over: Option<usize>,
}

fn get_location_of_cards(cards: Vec<Image>) -> Vec<(Image, Rectangle)> {
    cards
        .into_iter()
        .enumerate()
        .map(|(key, card)| {
            let rec_size = Vector::new(135.750_67, 192.);
            let rec_location = Vector::new(8.5375, 6.400_000_6 + (35.84 * key as f32));
            (card, Rectangle::new(rec_location, rec_size))
        })
        .collect()
}

impl Battle {
    pub(crate) async fn new(wrapper: &mut Wrapper) -> crate::Result<Battle> {
        let outer_radius = 307.200_000_000_000_05;
        let outer_points = calc_points(outer_radius, 8, 10.0, |x: f64, y: f64, _| {
            (x + 683.85375, y + 384.639_997_44 /*300.5f64*/)
        });
        //let inner_radius = ;

        let current = wrapper.client.new_battle(&wrapper.gfx).await?;
        let (current, hand) = (current.battle, current.images);
        let hand = get_location_of_cards(hand);

        let font = VectorFont::load("font.ttf").await?;

        Ok(Battle {
            player_mana: current.mana.to_string(),
            enemy_mana: current.enemy_mana.to_string(),
            outer_points,
            hand,
            clicked: false,
            enemy_hand_size: format!("S: {}", current.enemy_hand_size),
            enemy_hp: format!("HP: {}", current.enemy_hp),
            player_hp: format!("HP: {}", current.player_hp),
            enemy_runes: current.enemy_small_runes,
            player_runes: current.small_runes,
            stat_font: font.to_renderer(&wrapper.gfx, 25.0)?,
            hexa_runes: RuneAnimation::new(179.2),
            hover_over: None,
        })
    }
    fn get_card_hovering_over(&self, cursor_pos: Vector) -> Option<usize> {
        self.hand
            .iter()
            .enumerate()
            .rev()
            .map(|(key, card)| (key, card.1))
            .find(|(_, card)| card.contains(cursor_pos))
            .map(|(k, _)| k)
    }
    async fn play_card(&mut self, wrapper: &mut Wrapper) -> crate::Result<()> {
        let cursor_pos = wrapper.get_cursor_loc();
        let chosen = self.get_card_hovering_over(cursor_pos);
        if let Some(chosen) = chosen {
            let battle = wrapper.client.do_turn(chosen, &wrapper.gfx).await?;
            let (battle, hand) = (battle.battle, battle.images);
            self.hand = get_location_of_cards(hand);
            self.enemy_hand_size = format!("S: {}", battle.enemy_hand_size);
            self.enemy_hp = format!("HP: {}", battle.enemy_hp);
            self.player_hp = format!("HP: {}", battle.player_hp);
            self.enemy_runes = battle.enemy_small_runes;
            self.player_runes = battle.small_runes;
            self.enemy_mana = battle.enemy_mana.to_string();
            self.player_mana = battle.mana.to_string();
            self.hexa_runes.set_state(battle.hexa_runes);
            //self.hexa_runes = battle.hexa_runes;
            self.hover_over = self.get_card_hovering_over(cursor_pos);
        }
        Ok(())
    }
}

#[async_trait(?Send)]
impl Screen for Battle {
    async fn draw(&mut self, wrapper: &mut crate::Wrapper) -> crate::Result<()> {
        //let resolution = SIZE;
        wrapper.gfx.clear(Color::from_hex("#031234"));

        self.outer_points
            .iter()
            .enumerate()
            .for_each(|(key, circle)| {
                let rune = has_rune(key, &self.player_runes, &self.enemy_runes);
                match rune {
                    Some(_) => {
                        wrapper
                            .gfx
                            .fill_circle(circle, Color::from_rgba(key as u8 * 31, 0, 255, 1.0));
                    }
                    None => {
                        wrapper
                            .gfx
                            .fill_circle(circle, Color::from_rgba(255, 0, key as u8 * 31, 1.0));
                    }
                }
                wrapper.gfx.draw_point(circle.pos, Color::WHITE);
            });
        self.hexa_runes.draw(&mut wrapper.gfx);
        wrapper.gfx.fill_circle(
            &Circle::new(Vector::new(SIZE.x / 2f32, SIZE.y / 2f32), 20.),
            Color::WHITE,
        );
        wrapper
            .gfx
            .stroke_path(&[(0., 0.).into(), SIZE], Color::BLUE);

        for (card, rectangle) in self.hand.iter() {
            wrapper.gfx.draw_image(card, *rectangle);
        }
        if let Some(card) = self.hover_over.and_then(|v| self.hand.get(v)) {
            wrapper.gfx.draw_image(
                &card.0,
                Rectangle::new(
                    Vector::new(card.1.pos.x + card.1.size.x + 5., card.1.pos.y),
                    card.1.size() * 1.2,
                ),
            );
        }
        let renderer = &mut self.stat_font;
        let offset = Vector::new(27.32, 729.6);
        renderer.draw(&mut wrapper.gfx, &self.player_hp, Color::RED, offset)?;
        let offset = Vector::new(27.32, 691.2); // wrapper.get_pos_vector(0.02, 0.90);
        renderer.draw(&mut wrapper.gfx, &self.player_mana, Color::RED, offset)?;
        let offset = Vector::new(1256.72, 38.4); //wrapper.get_pos_vector(0.92, 0.05);
        renderer.draw(&mut wrapper.gfx, &self.enemy_hp, Color::RED, offset)?;

        let offset = Vector::new(1256.72, 76.8); //wrapper.get_pos_vector(0.92, 0.1);
        renderer.draw(&mut wrapper.gfx, &self.enemy_hand_size, Color::RED, offset)?;
        let offset = Vector::new(1256.72, 115.2);
        renderer.draw(&mut wrapper.gfx, &self.enemy_mana, Color::RED, offset)?;
        Ok(())
    }
    async fn update(&mut self, _: &mut crate::Wrapper) -> crate::Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }

    async fn event(
        &mut self,
        wrapper: &mut Wrapper,
        event: &quicksilver::input::Event,
    ) -> crate::Result<Option<Box<dyn Screen>>> {
        use quicksilver::input::{Event::*, MouseButton};
        match event {
            PointerMoved(_) => {
                self.hover_over = self.get_card_hovering_over(wrapper.cursor_at);
            }

            PointerInput(x) if x.button() == MouseButton::Left => {
                if x.is_down() {
                    if !self.clicked {
                        self.clicked = true;
                        self.play_card(wrapper).await?;
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
