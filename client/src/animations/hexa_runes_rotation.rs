use quicksilver::{
    geom::{Circle, Vector},
    graphics::Color,
    Graphics, Result, Timer,
};
use silver_animation::{AnimationTimer, ContainedAnimation, EditableState};
use std::f64::consts::PI;

pub fn calc_points(
    radius: f64,
    points: i8,
    rotation: f64,
    offset: impl Fn(f64, f64, f64) -> (f64, f64),
) -> Vec<Circle> {
    let radius: f64 = radius;
    let steps: f64 = 2.0 * PI / f64::from(points); //0.78539816339;
                                                   //let steps: f64 = 1.0471975512;
    (0..points)
        .map(|v| f64::from(v) * steps + rotation)
        .map(|v| (radius * (v.sin()), radius * (v.cos())))
        .map(|(x, y)| offset(x, y, radius))
        .map(|(x, y)| Circle::new(Vector::new(x as f32, y as f32), 35.))
        .collect()
}

pub struct RuneAnimation {
    rune_amount: i8,
    rune_info: Vec<String>,
    step_size: f64,
    animation_timer: AnimationTimer<f64, fn(&f64) -> usize>,
    location: Circle,
}
//we need the max_frames to take by reference to fit the required definition of the AnimationTimer.
//does it suck? yea, kinda
#[allow(clippy::trivially_copy_pass_by_ref)]
fn max_frames(step_size: &f64) -> usize {
    (360f64 / step_size).floor() as usize
}

impl RuneAnimation {
    pub fn new(radius: f32) -> Self {
        Self {
            rune_amount: 5,
            rune_info: Vec::new(),
            step_size: 0.0005,
            animation_timer: AnimationTimer::new(max_frames, Timer::time_per_second(60.)),
            location: Circle::new(Vector::new(0., 0.), radius),
        }
    }
    pub fn set_state(&mut self, state: Vec<String>) {
        <Self as EditableState<_>>::set_state(self, state)
    }
    pub fn draw(&mut self, gfx: &mut Graphics) {
        let _ = <Self as ContainedAnimation<_, _>>::draw(self, gfx);
    }
}

impl ContainedAnimation<f32, Circle> for RuneAnimation {
    fn draw(&mut self, gfx: &mut Graphics) -> Result<()> {
        let rotation =
            self.animation_timer.get_current_frame(&self.step_size) as f64 * self.step_size;
        let runes = calc_points(
            self.location.radius.into(),
            self.rune_amount,
            rotation,
            |x, y, _| (x + 683.85375, y + 384.639_997_44),
        );
        runes
            .iter()
            .enumerate()
            .map(|(key, circle)| {
                (
                    circle,
                    if self.rune_info.get(key).is_some() {
                        Color::from_rgba(255, key as u8 * 63, 0, 1.0)
                    } else {
                        Color::from_rgba(0, 255, key as u8 * 63, 1.0)
                    },
                )
            })
            .for_each(|(circle, color)| {
                gfx.fill_circle(circle, color);
                gfx.draw_point(circle.pos, Color::WHITE);
            });
        Ok(())
    }
    fn set_location(&mut self, location: Vector) {
        self.location.pos = location;
    }
    fn set_size(&mut self, size: f32) {
        self.location.radius = size;
    }
    fn get_draw_pos(&self) -> Circle {
        self.location
    }
    fn get_position(&self) -> Vector {
        self.location.pos
    }
    fn get_size(&self) -> f32 {
        self.location.radius
    }
}

impl EditableState<Vec<String>> for RuneAnimation {
    fn set_state(&mut self, new_state: Vec<String>) {
        self.rune_info = new_state;
    }
    fn get_state(&self) -> &Vec<String> {
        &self.rune_info
    }
}

/*
pub struct HexaRunesRotation {
    center: Vector,
    size: Vector,
    rune_locations: Vec<Circle>,
    rune_info: Vec<String>,
    time: Timer,
    frame: usize,
}

impl ContainedAnimation for HexaRunesRotation {
    fn draw(&mut self, gfx: &mut quicksilver::Graphics) -> quicksilver::Result<()> {
        let frames_passed = self.config.timing.exhaust().map(usize::from).unwrap_or(0);
        match frames_passed.checked_add(self.last_frame) {
            Some(x) => {
                self.last_frame = x % (self.config.max_frames)(&self.config.begin_state);
            }
            None => {
                let max_size = (self.config.max_frames)(&self.config.begin_state);
                let bound_to_frame = frames_passed % max_size;
                self.last_frame = (bound_to_frame + self.last_frame) % max_size;
            }
        }
        self.rune_locations
            .iter()
            .enumerate()
            .map(|(key, circle)| {
                (
                    circle,
                    if self.rune_info.get(key).is_some() {
                        Color::from_rgba(255, key as u8 * 63, 0, 1.0)
                    } else {
                        Color::from_rgba(0, 255, key as u8 * 63, 1.0)
                    },
                )
            })
            .for_each(|(circle, color)| {
                gfx.fill_circle(circle, color);
                gfx.draw_point(circle.pos, Color::WHITE);
            });
        Ok(())
    }
    fn set_location(&mut self, location: quicksilver::geom::Vector) {
        self.center = location;
    }
    fn set_size(&mut self, size: quicksilver::geom::Vector) {
        self.size = size;
    }
    fn get_draw_pos(&self) -> quicksilver::geom::Rectangle {
        Rectangle::new(self.center, self.size)
    }
    fn get_position(&self) -> quicksilver::geom::Vector {
        self.center
    }
    fn get_size(&self) -> quicksilver::geom::Vector {
        self.size
    }
}
*/
