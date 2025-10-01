use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::vehicles;

const SCREEN_W: u32 = 1000;
const SCREEN_H: u32 = 800;

const IX_MIN: i32 = 425;
const IX_MAX: i32 = 575;
const IY_MIN: i32 = 325;
const IY_MAX: i32 = 475;

#[derive(Clone, Copy)]
pub enum Route {
    TurnLeft,
    GoStraight,
    TurnRight,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LightState {
    Green,
    Red,
}

pub struct TrafficLight {
    pub position: (i32, i32),
    pub state: LightState,
    pub approach: vehicles::Approach,
}

pub fn draw_roads(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, lights: &[TrafficLight]) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let _ = canvas.draw_line((500, 0), (500, IY_MIN));
    let _ = canvas.draw_line((500, IY_MAX), (500, SCREEN_H as i32));
    let _ = canvas.draw_line((575, 0), (575, IY_MIN));
    let _ = canvas.draw_line((575, IY_MAX), (575, SCREEN_H as i32));
    let _ = canvas.draw_line((425, 0), (425, IY_MIN));
    let _ = canvas.draw_line((425, IY_MAX), (425, SCREEN_H as i32));

    let _ = canvas.draw_line((0, 400), (IX_MIN, 400));
    let _ = canvas.draw_line((IX_MAX, 400), (SCREEN_W as i32, 400));
    let _ = canvas.draw_line((0, 325), (IX_MIN, 325));
    let _ = canvas.draw_line((IX_MAX, 325), (SCREEN_W as i32, 325));
    let _ = canvas.draw_line((0, 475), (IX_MIN, 475));
    let _ = canvas.draw_line((IX_MAX, 475), (SCREEN_W as i32, 475));

    for light in lights {
        canvas.set_draw_color(match light.state {
            LightState::Green => Color::RGB(0, 255, 0),
            LightState::Red => Color::RGB(255, 0, 0),
        });
        let light_rect = Rect::new(light.position.0 - 5, light.position.1 - 5, 40, 40);
        let _ = canvas.fill_rect(light_rect);
    }
}

pub fn rand_route(rng: &mut rand::prelude::ThreadRng) -> Route {
    match rng.gen_range(0..3) {
        0 => Route::TurnLeft,
        1 => Route::GoStraight,
        _ => Route::TurnRight,
    }
}
