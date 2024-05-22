mod vehicule;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::image::{self, LoadTexture};
use sdl2::rect::Rect;
use vehicule::{Vehicle, Direction};

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

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(sdl2::keyboard::Keycode::Up) => {
                            vehicles.push(Vehicle::new(Direction::North, 5, 565, 900));
                        },
                        Some(sdl2::keyboard::Keycode::Down) => {
                            vehicles.push(Vehicle::new(Direction::South, 5, 410, 00));
                        },
                        Some(sdl2::keyboard::Keycode::Right) => {
                            vehicles.push(Vehicle::new(Direction::East, 5, 0, 445));
                        },
                        Some(sdl2::keyboard::Keycode::Left) => {
                            vehicles.push(Vehicle::new(Direction::West, 5, 1000, 310));
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
            let vehicle_texture = match vehicle.direction {
                Direction::North => &vehicle_texture_up,
                Direction::South => &vehicle_texture_down,
                Direction::East => &vehicle_texture_right,
                Direction::West => &vehicle_texture_left,
            };
        
            // Set different dimensions based on direction
            let (width, height) = match vehicle.direction {
                Direction::North | Direction::South => (30, 60),
                Direction::East | Direction::West => (60, 30),
            };
        
            let vehicle_rect = Rect::new(vehicle.x, vehicle.y, width, height);
            canvas.copy(vehicle_texture, None, vehicle_rect).unwrap();
            vehicle.move_car();
        }
        

        canvas.present();
    }

    // Clean up SDL2_image
    // image::quit();
}
