use super::Screen;
use async_trait::async_trait;
use card_game_shared::{BasicVector, dungeon};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color, input::Event, blinds::Key,
};
use crate::{check_multiple, Wrapper};

const BASE_DUNGEON_SIZE_X: u64 = 10;
const BASE_DUNGEON_SIZE_Y: u64 = 10;
const BASE_DUNGEON_CARD_SIZE_X: f32 = 70.;
const BASE_DUNGEON_CARD_SIZE_Y: f32 = 66.;
const DISTANCE_BETWEEN_CARDS: f32 = 2.;

//#[const_tweaker::tweak(min = 0., max = 1000.)]
const MARGIN_DUNGEON_X: f32 = 510.;

//#[const_tweaker::tweak(min = 0., max = 100.)]
const MARGIN_DUNGEON_Y: f32 = 42.;

const PLAYER_SIZE: f32 = 30.;
pub struct Dungeon {
    state: dungeon::DungeonLayout,
    char_id : i64
}
impl Dungeon {
    pub(crate) async fn new(char_id: i64, wrapper: &mut crate::Wrapper) -> crate::Result<Self> {
        wrapper
            .client
            .get_dungeon(char_id)
            .await
            .map(|state| Self { state,char_id })
    }
}
#[async_trait(?Send)]
impl Screen for Dungeon {
    async fn draw(&mut self, wrapper: &mut crate::Wrapper) -> crate::Result<()> {
        wrapper.gfx.clear(Color::BLUE);
        let card_size_y: f32 = (BASE_DUNGEON_CARD_SIZE_Y + DISTANCE_BETWEEN_CARDS)
            / (self.state.height as f64 / BASE_DUNGEON_SIZE_Y as f64) as f32
            - DISTANCE_BETWEEN_CARDS;

        let card_size_x = (BASE_DUNGEON_CARD_SIZE_X + DISTANCE_BETWEEN_CARDS)
            / (self.state.widht as f64 / BASE_DUNGEON_SIZE_X as f64) as f32
            - DISTANCE_BETWEEN_CARDS;
        let card_size = Vector::new(card_size_x, card_size_y);

        for (loc, state) in self.state.tiles.iter().enumerate() {
            let card_loc_x = (loc % self.state.widht);
            let card_loc_y = (loc / self.state.widht);
            let draw_player =
                card_loc_x == self.state.player_at.x && card_loc_y == self.state.player_at.y;

            let card_loc_x = card_loc_x as f32;
            let card_loc_y = card_loc_y as f32;

            let card_rectangle = Rectangle::new(
                Vector::new(
                    card_loc_x * card_size_x
                        + (card_loc_x * DISTANCE_BETWEEN_CARDS)
                        + MARGIN_DUNGEON_X,
                    card_loc_y * card_size_y
                        + (card_loc_y * DISTANCE_BETWEEN_CARDS)
                        + MARGIN_DUNGEON_Y,
                ),
                card_size,
            );
            let color = match state {
                dungeon::TileState::Seen(_) => {Color::WHITE}
                dungeon::TileState::Empty => {Color::BLUE}
                dungeon::TileState::Hidden => {Color::BLACK}
            };
            wrapper.gfx.fill_rect(&card_rectangle, color);
            if draw_player {
                wrapper.gfx.fill_rect(
                    &Rectangle::new(
                        card_rectangle.pos + (card_rectangle.size / 2.)
                            - Vector::new(PLAYER_SIZE / 2., PLAYER_SIZE / 2.),
                        Vector::new(PLAYER_SIZE, PLAYER_SIZE),
                    ),
                    Color::GREEN,
                )
            }
        }
        Ok(())
    }
    async fn update(&mut self, _: &mut crate::Wrapper) -> crate::Result<Option<Box<dyn Screen>>> {
        Ok(None)
    }
    async fn event(
            &mut self,
            wrapper: &mut Wrapper,
            event: &Event,
        ) -> crate::Result<Option<Box<dyn Screen>>> {
        match event {
            Event::KeyboardInput(x) => {
                let dir = if check_multiple(x,&[Key::Up,Key::W]){
                    BasicVector {x:0,y:-1}
                } else if check_multiple(x, &[Key::A,Key::Left]) {
                    BasicVector {x: -1,y:0}
                } else if check_multiple(x, &[Key::S,Key::Down]) {
                    BasicVector {x:0,y:1}
                } else if check_multiple(x, &[Key::D,Key::Right]) {
                    BasicVector {x:1,y:0}
                } else {
                    return Ok(None);
                };

                match wrapper.client.move_in_dungeon(self.char_id, dir).await? {
                    dungeon::EventProcesed::Success => {self.state = wrapper.client.get_dungeon(self.char_id).await?}
                    dungeon::EventProcesed::Error | dungeon::EventProcesed::CurrentlyInBattle => {

                    }
                }
            }
            //should be implemented so you can click on cards to move as well as use the keyboard
            Event::PointerInput(_) => {
                return Ok(None)
            }
            _ => ()
        }
        Ok(None)
    }
}
