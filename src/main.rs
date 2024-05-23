mod vehicule;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture};
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vehicule::{Vehicle, Direction};

const OFFSETS: [i32; 3] = [0, 25, -25];

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SMART____ROAD", 1000, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Initialize SDL2_image
    image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();

    // Load the background image
    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture("./assets/road.jpg").unwrap();

    // Load the vehicle textures
    let vehicle_texture_up = texture_creator.load_texture("./assets/car-up.png").unwrap();
    let vehicle_texture_down = texture_creator.load_texture("./assets/car-down.png").unwrap();
    let vehicle_texture_left = texture_creator.load_texture("./assets/car-left.png").unwrap();
    let vehicle_texture_right = texture_creator.load_texture("./assets/car-right.png").unwrap();

    // Vector to store vehicles
    let mut vehicles: Vec<Vehicle> = Vec::new();

    // Variables to keep track of the last used offset index for each direction
    let mut offset_index_x = 0;
    let mut offset_index_y = 0;

    // Track sequence for vehicles going south
    let mut south_sequence = 0;

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::Up) => {
                            vehicles.push(Vehicle::new(Direction::North, 5, 565 + OFFSETS[offset_index_y as usize], 900, 0));
                            offset_index_y = (offset_index_y + 1) % 3;
                        },
                        Some(sdl2::keyboard::Keycode::Down) => {
                            vehicles.push(Vehicle::new(Direction::South, 5, 410 + OFFSETS[offset_index_y as usize], 0, south_sequence));
                            offset_index_y = (offset_index_y + 1) % 3;
                            south_sequence = (south_sequence + 1) % 3;
                        },
                        Some(sdl2::keyboard::Keycode::Right) => {
                            vehicles.push(Vehicle::new(Direction::East, 5, 0, 445 + OFFSETS[offset_index_x as usize], 0));
                            offset_index_x = (offset_index_x + 1) % 3;
                        },
                        Some(sdl2::keyboard::Keycode::Left) => {
                            vehicles.push(Vehicle::new(Direction::West, 5, 1000, 445 + OFFSETS[offset_index_x as usize], 0));
                            offset_index_x = (offset_index_x + 1) % 3;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Copy the texture to the canvas
        canvas.copy(&background_texture, None, None).unwrap();

        // Update and draw all vehicles
        for vehicle in &mut vehicles {
            let (vehicle_texture, angle) = match vehicle.direction {
                Direction::North => (&vehicle_texture_up, 0.0),
                Direction::South => {
                    match vehicle.turn_counter {
                        0 if vehicle.y > 500 => (&vehicle_texture_right, 90.0),  // Turn right
                        2 if vehicle.y > 500 => (&vehicle_texture_left, -90.0), // Turn left
                        _ => (&vehicle_texture_down, 0.0),
                    }
                },
                Direction::East => (&vehicle_texture_right, 0.0),
                Direction::West => (&vehicle_texture_left, 0.0),
            };

            // Set different dimensions based on direction
            let (width, height) = match vehicle.direction {
                Direction::North | Direction::South => (20, 50),
                Direction::East | Direction::West => (50, 20),
            };

            vehicle.move_car();

            let vehicle_rect = Rect::new(vehicle.x, vehicle.y, width, height);
            canvas.copy_ex(vehicle_texture, None, vehicle_rect, angle, None, false, false).unwrap();
        }

        canvas.present();
    }

    // Clean up SDL2_image
    // image::quit();
}
