use crate::Wrapper;
use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::Image,
};

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

pub(crate) struct Hand {
    hover_over: Option<usize>,
    hand: Vec<(Image, Rectangle)>,
    clicked: bool,
}

impl Hand {
    pub(crate) fn new() -> Self {
        Self {
            hand: Vec::new(),
            hover_over: None,
            clicked: false,
        }
    }
    fn set_card_hovering_over(&mut self, cursor_pos: Vector) {
        self.hover_over = self
            .hand
            .iter()
            .enumerate()
            .rev()
            .map(|(key, card)| (key, card.1))
            .find(|(_, card)| card.contains(cursor_pos))
            .map(|(k, _)| k)
    }
    pub(crate) fn update_hand(&mut self, cards: Vec<Image>, wrapper: &Wrapper) {
        self.hand = get_location_of_cards(cards);
        self.set_card_hovering_over(wrapper.cursor_at)
    }
    pub(crate) fn event(
        &mut self,
        event: &quicksilver::input::Event,
        wrapper: &mut Wrapper,
    ) -> Option<usize> {
        use quicksilver::input::{Event::*, MouseButton};
        match event {
            PointerMoved(_) => {
                self.set_card_hovering_over(wrapper.cursor_at());
            }

            PointerInput(x) if x.button() == MouseButton::Left => {
                if x.is_down() {
                    if !self.clicked {
                        self.clicked = true;
                        return self.hover_over;
                    }
                } else {
                    self.clicked = false;
                }
            }
            _ => {}
        }
        None
    }
    pub(crate) fn draw(&self, wrapper: &mut Wrapper) {
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
    }
}
