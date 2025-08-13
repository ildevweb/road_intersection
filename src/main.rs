use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy)]
enum Route {
    Left,
    Right,
    Straight,
}

struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    route: Route,
    color: Color,
    speed: f32,
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window = video_subsystem
        .window("Traffic Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut last_spawn_time = Instant::now();
    let spawn_delay = Duration::from_millis(1000);
    let mut vehicles: Vec<Vehicle> = Vec::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    let now = Instant::now();
                    if now.duration_since(last_spawn_time) > spawn_delay {
                        let mut spawn_vehicle = |direction: Direction| {
                            let lane_offset = 15.0; // distance from center line

                            let (x, y) = match direction {
                                Direction::North => ((WINDOW_WIDTH / 2) as f32 + lane_offset, WINDOW_HEIGHT as f32),
                                Direction::South => ((WINDOW_WIDTH / 2) as f32 - lane_offset*2.0, 0.0 - 40.0),
                                Direction::East  => (0.0 - 40.0, (WINDOW_HEIGHT / 2) as f32 + lane_offset),
                                Direction::West  => (WINDOW_WIDTH as f32, (WINDOW_HEIGHT / 2) as f32 - lane_offset*2.0),
                            };


                            let route = match rand::thread_rng().gen_range(0..3) {
                                0 => Route::Left,
                                1 => Route::Right,
                                _ => Route::Straight,
                            };

                            let color = match route {
                                Route::Left => Color::RGB(255, 255, 0),   // Yellow
                                Route::Right => Color::RGB(0, 255, 0),    // Green
                                Route::Straight => Color::RGB(0, 0, 255), // Blue
                            };

                            vehicles.push(Vehicle {
                                x,
                                y,
                                direction,
                                route,
                                color,
                                speed: 2.0,
                            });
                        };

                        match kc {
                            Keycode::Up => spawn_vehicle(Direction::North),
                            Keycode::Down => spawn_vehicle(Direction::South),
                            Keycode::Left => spawn_vehicle(Direction::East),
                            Keycode::Right => spawn_vehicle(Direction::West),
                            Keycode::R => {
                                let dir = rand::thread_rng().gen_range(0..4);
                                let random_dir = match dir {
                                    0 => Direction::North,
                                    1 => Direction::South,
                                    2 => Direction::East,
                                    _ => Direction::West,
                                };
                                spawn_vehicle(random_dir);
                            }
                            _ => {}
                        }

                        last_spawn_time = now;
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_intersection(&mut canvas)?;
        update_and_draw_vehicles(&mut canvas, &mut vehicles)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}


fn update_and_draw_vehicles(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    vehicles: &mut Vec<Vehicle>,
) -> Result<(), String> {
    use sdl2::rect::Rect;

    // Dimensions
    let (vehicle_w, vehicle_h) = (20, 40);

    // Keep only vehicles that are on screen
    vehicles.retain(|v| {
        v.x >= -50.0 && v.x <= WINDOW_WIDTH as f32 + 50.0 &&
        v.y >= -50.0 && v.y <= WINDOW_HEIGHT as f32 + 50.0
    });

    for vehicle in vehicles.iter_mut() {
        // Move vehicle
        match vehicle.direction {
            Direction::North => vehicle.y -= vehicle.speed,
            Direction::South => vehicle.y += vehicle.speed,
            Direction::East  => vehicle.x += vehicle.speed,
            Direction::West  => vehicle.x -= vehicle.speed,
        }

        // Adjust shape for direction
        let (w, h) = match vehicle.direction {
            Direction::North | Direction::South => (vehicle_w, vehicle_h),
            Direction::East | Direction::West => (vehicle_h, vehicle_w),
        };

        // Draw
        canvas.set_draw_color(vehicle.color);
        let rect = Rect::new(vehicle.x as i32, vehicle.y as i32, w, h);
        canvas.fill_rect(rect)?;
    }

    Ok(())
}


fn draw_intersection(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
    use sdl2::rect::Rect;
    use sdl2::pixels::Color;

    let road_width = 100;
    let line_width = 1;
    let light_size = 20;
    let margin = 5;

    let center_x = (WINDOW_WIDTH / 2) as i32;
    let center_y = (WINDOW_HEIGHT / 2) as i32;

    // --- Draw Roads ---
    canvas.set_draw_color(Color::RGB(50, 50, 50));

    // Vertical road
    canvas.fill_rect(Rect::new(center_x - road_width / 2, 0, road_width as u32, WINDOW_HEIGHT))?;

    // Horizontal road
    canvas.fill_rect(Rect::new(0, center_y - road_width / 2, WINDOW_WIDTH, road_width as u32))?;

    // --- Draw Center Divider Lines ---
    canvas.set_draw_color(Color::RGB(255, 255, 0));

    // Vertical center line
    canvas.fill_rect(Rect::new(center_x - line_width / 2, 0, line_width as u32, WINDOW_HEIGHT))?;

    // Horizontal center line
    canvas.fill_rect(Rect::new(0, center_y - line_width / 2, WINDOW_WIDTH, line_width as u32))?;

    // --- Draw Traffic Lights (red borders) ---
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    // Top-left corner
    let top_left = Rect::new(
        center_x - road_width / 2 - light_size - margin,
        center_y - road_width / 2 - light_size - margin,
        light_size as u32,
        light_size as u32,
    );
    canvas.draw_rect(top_left)?;

    // Top-right corner
    let top_right = Rect::new(
        center_x + road_width / 2 + margin,
        center_y - road_width / 2 - light_size - margin,
        light_size as u32,
        light_size as u32,
    );
    canvas.draw_rect(top_right)?;

    // Bottom-left corner
    let bottom_left = Rect::new(
        center_x - road_width / 2 - light_size - margin,
        center_y + road_width / 2 + margin,
        light_size as u32,
        light_size as u32,
    );
    canvas.draw_rect(bottom_left)?;

    // Bottom-right corner
    let bottom_right = Rect::new(
        center_x + road_width / 2 + margin,
        center_y + road_width / 2 + margin,
        light_size as u32,
        light_size as u32,
    );
    canvas.draw_rect(bottom_right)?;

    Ok(())
} 
