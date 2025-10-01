use crate::roads::{LightState, Route, TrafficLight};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const VEH_LEN: i32 = 50;
const VEH_WID: i32 = 50;
const SPEED: i32 = 2;
const SAFE_GAP: i32 = 60;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Approach {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Vehicle {
    pub rect: Rect,
    pub dir: Approach,
    pub route: Route,
    pub color: Color,
    pub speed: i32,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, dir: Approach, route: Route) -> Self {
        let color = match route {
            Route::TurnRight => Color::RGB(255, 220, 0),
            Route::GoStraight => Color::RGB(66, 135, 245),
            Route::TurnLeft => Color::RGB(220, 20, 60),
        };
        Vehicle {
            rect: Rect::new(x, y, VEH_WID as u32, VEH_LEN as u32),
            dir,
            route,
            color,
            speed: SPEED,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.rect);
    }

    pub fn can_move(&self, lights: &[TrafficLight]) -> bool {
        for light in lights {
            if light.approach == self.dir && light.state == LightState::Red {
                match self.dir {
                    Approach::Up => {
                        if self.rect.y <= 485 && self.rect.y >= 475 {
                            return false;
                        }
                    }
                    Approach::Down => {
                        if self.rect.y + VEH_LEN >= 285 && self.rect.y + VEH_LEN <= 295 {
                            return false;
                        }
                    }
                    Approach::Left => {
                        if self.rect.x <= 585 && self.rect.x >= 575 {
                            return false;
                        }
                    }
                    Approach::Right => {
                        if self.rect.x + VEH_WID >= 385 && self.rect.x + VEH_WID <= 395 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn step(&mut self, lights: &[TrafficLight]) {
        if !self.can_move(lights) {
            return;
        }
        match self.route {
            Route::GoStraight => match self.dir {
                Approach::Up => self.rect.y -= self.speed,
                Approach::Down => self.rect.y += self.speed,
                Approach::Left => self.rect.x -= self.speed,
                Approach::Right => self.rect.x += self.speed,
            },
            Route::TurnRight => match self.dir {
                Approach::Up => {
                    if self.rect.y <= 415 {
                        self.rect.x += self.speed;
                    } else {
                        self.rect.y -= self.speed;
                    }
                }
                Approach::Down => {
                    if self.rect.y >= 340 {
                        self.rect.x -= self.speed;
                    } else {
                        self.rect.y += self.speed;
                    }
                }
                Approach::Left => {
                    if self.rect.x >= 515 {
                        self.rect.x -= self.speed;
                    } else {
                        self.rect.y -= self.speed;
                    }
                }
                Approach::Right => {
                    if self.rect.x <= 435 {
                        self.rect.x += self.speed;
                    } else {
                        self.rect.y += self.speed;
                    }
                }
            },
            Route::TurnLeft => match self.dir {
                Approach::Up => {
                    if self.rect.y <= 340 {
                        self.rect.x -= self.speed;
                    } else {
                        self.rect.y -= self.speed;
                    }
                }
                Approach::Down => {
                    if self.rect.y >= 410 {
                        self.rect.x += self.speed;
                    } else {
                        self.rect.y += self.speed;
                    }
                }
                Approach::Left => {
                    if self.rect.x >= 440 {
                        self.rect.x -= self.speed;
                    } else {
                        self.rect.y += self.speed;
                    }
                }
                Approach::Right => {
                    if self.rect.x <= 510 {
                        self.rect.x += self.speed;
                    } else {
                        self.rect.y -= self.speed;
                    }
                }
            },
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.rect.x < -75 || self.rect.x > 1075 || self.rect.y < -75 || self.rect.y > 875
    }
}

pub fn can_spawn_vehicle(
    vehicles: &Vec<Vehicle>,
    spawn_x: i32,
    spawn_y: i32,
    dir: Approach,
) -> bool {
    for v in vehicles {
        match dir {
            Approach::Up => {
                if v.dir == Approach::Up && (spawn_y - v.rect.y).abs() < SAFE_GAP {
                    return false;
                }
            }
            Approach::Down => {
                if v.dir == Approach::Down && (v.rect.y - spawn_y).abs() < SAFE_GAP {
                    return false;
                }
            }
            Approach::Left => {
                if v.dir == Approach::Left && (spawn_x - v.rect.x).abs() < SAFE_GAP {
                    return false;
                }
            }
            Approach::Right => {
                if v.dir == Approach::Right && (v.rect.x - spawn_x).abs() < SAFE_GAP {
                    return false;
                }
            }
        }
    }
    true
}
