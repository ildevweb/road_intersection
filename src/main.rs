mod roads;
mod vehicles;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const SCREEN_W: u32 = 1000;
const SCREEN_H: u32 = 800;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("road intersection", SCREEN_W, SCREEN_H)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut vehicles: Vec<vehicles::Vehicle> = Vec::new();
    let mut lights = vec![
        // still need to check all the cases here
        roads::TrafficLight {
            position: (585, 485),
            state: roads::LightState::Green,
            approach: vehicles::Approach::Up,
        }, // here for up
        roads::TrafficLight {
            position: (385, 285),
            state: roads::LightState::Green,
            approach: vehicles::Approach::Down,
        }, // down
        roads::TrafficLight {
            position: (585, 285),
            state: roads::LightState::Red,
            approach: vehicles::Approach::Left,
        }, // left
        roads::TrafficLight {
            position: (385, 485),
            state: roads::LightState::Red,
            approach: vehicles::Approach::Right,
        }, // right
    ];
    let mut cd_up: u32 = 0;
    let mut cd_down: u32 = 0;
    let mut cd_left: u32 = 0;
    let mut cd_right: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let dir = vehicles::Approach::Up;
                    let (sx, sy) = (515, 750);
                    if cd_up == 0 && vehicles::can_spawn_vehicle(&vehicles, sx, sy, dir) {
                        let mut rng = rand::thread_rng();
                        let v = vehicles::Vehicle::new(sx, sy, dir, roads::rand_route(&mut rng));
                        vehicles.push(v);
                        cd_up = 18;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let dir = vehicles::Approach::Down;
                    let (sx, sy) = (440, 0);
                    if cd_down == 0 && vehicles::can_spawn_vehicle(&vehicles, sx, sy, dir) {
                        let mut rng = rand::thread_rng();
                        let v = vehicles::Vehicle::new(sx, sy, dir, roads::rand_route(&mut rng));
                        vehicles.push(v);
                        cd_down = 18;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let dir = vehicles::Approach::Left;
                    let (sx, sy) = (950, 335);
                    if cd_left == 0 && vehicles::can_spawn_vehicle(&vehicles, sx, sy, dir) {
                        let mut rng = rand::thread_rng();
                        let v = vehicles::Vehicle::new(sx, sy, dir, roads::rand_route(&mut rng));
                        vehicles.push(v);
                        cd_left = 18;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let dir = vehicles::Approach::Right;
                    let (sx, sy) = (10, 415);
                    if cd_right == 0 && vehicles::can_spawn_vehicle(&vehicles, sx, sy, dir) {
                        let mut rng = rand::thread_rng();
                        let v = vehicles::Vehicle::new(sx, sy, dir, roads::rand_route(&mut rng));
                        vehicles.push(v);
                        cd_right = 18;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let dirs = [
                        vehicles::Approach::Up,
                        vehicles::Approach::Down,
                        vehicles::Approach::Left,
                        vehicles::Approach::Right,
                    ];
                    let spawn_points = [(515, 750), (440, 0), (950, 335), (10, 415)];
                    let mut rng = rand::thread_rng();
                    let idx = rng.gen_range(0..4);
                    let dir = dirs[idx];
                    let (sx, sy) = spawn_points[idx];
                    let cd = match dir {
                        vehicles::Approach::Up => &mut cd_up,
                        vehicles::Approach::Down => &mut cd_down,
                        vehicles::Approach::Left => &mut cd_left,
                        vehicles::Approach::Right => &mut cd_right,
                    };
                    if *cd == 0 && vehicles::can_spawn_vehicle(&vehicles, sx, sy, dir) {
                        let v = vehicles::Vehicle::new(sx, sy, dir, roads::rand_route(&mut rng));
                        vehicles.push(v);
                        *cd = 18;
                    }
                }
                _ => {}
            }
        }

        // Update cooldowns
        if cd_up > 0 {
            cd_up -= 1;
        }
        if cd_down > 0 {
            cd_down -= 1;
        }
        if cd_left > 0 {
            cd_left -= 1;
        }
        if cd_right > 0 {
            cd_right -= 1;
        }

        // Update traffic lights
        let up_count = vehicles
            .iter()
            .filter(|v| v.dir == vehicles::Approach::Up)
            .count();
        let down_count = vehicles
            .iter()
            .filter(|v| v.dir == vehicles::Approach::Down)
            .count();
        let left_count = vehicles
            .iter()
            .filter(|v| v.dir == vehicles::Approach::Left)
            .count();
        let right_count = vehicles
            .iter()
            .filter(|v| v.dir == vehicles::Approach::Right)
            .count();

        let max_count = up_count.max(down_count).max(left_count.max(right_count));
        let green_approach = if up_count == max_count {
            vehicles::Approach::Up
        } else if down_count == max_count {
            vehicles::Approach::Down
        } else if left_count == max_count {
            vehicles::Approach::Left
        } else {
            vehicles::Approach::Right
        };

        for light in &mut lights {
            light.state = if light.approach == green_approach {
                roads::LightState::Green
            } else {
                roads::LightState::Red
            };
        }

        // Compute counts per approach
        let mut counts: [usize; 4] = [0, 0, 0, 0];
        let approaches = [
            vehicles::Approach::Up,
            vehicles::Approach::Down,
            vehicles::Approach::Left,
            vehicles::Approach::Right,
        ];
        for v in &vehicles {
            match v.dir {
                vehicles::Approach::Up => counts[0] += 1,
                vehicles::Approach::Down => counts[1] += 1,
                vehicles::Approach::Left => counts[2] += 1,
                vehicles::Approach::Right => counts[3] += 1,
            }
        }
        let max_count = *counts.iter().max().unwrap_or(&0);
        let max_idx = counts.iter().position(|&c| c == max_count).unwrap_or(0);
        let max_approach = approaches[max_idx];

        // Set lights: green for max approach, red for others
        for light in &mut lights {
            light.state = if light.approach == max_approach {
                roads::LightState::Green
            } else {
                roads::LightState::Red
            };
        }

        // Update vehicle positions
        for i in 0..vehicles.len() {
            vehicles[i].step(&lights);
        }

        // Remove off-screen vehicles
        vehicles.retain(|v| !v.is_off_screen());

        // Drawing
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        roads::draw_roads(&mut canvas, &lights);

        for v in &vehicles {
            v.draw(&mut canvas);
        }

        canvas.present();

        ::std::thread::sleep(Duration::from_millis(10));
    }
}
