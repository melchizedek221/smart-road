use sdl2::rect::Rect;
use sdl2::{render::Canvas, video::Window};
use crate::cars::{self, Vehicle};
use crate::statistics::Stats;
use sdl2::image::{self, LoadTexture};

#[derive(Debug, Clone, Copy)]
pub enum Cross {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
    pub vehicle_spacing: i32,
    pub cross: Cross,
    pub is_vehicles_stopped: bool,
    pub min_speed: u32,
    pub max_speed: u32,
    pub total_cars: u32,
    pub max_time: u64,
    pub min_time: u64,
    pub average_speed: u32,
    pub close_calls: u32,
}

impl Lane {
    pub fn new(vehicle_spacing: i32, cross: Cross) -> Lane {
        Lane {
            vehicles: Vec::new(),
            vehicle_spacing,
            cross,
            is_vehicles_stopped: false,
            min_speed: 0,
            max_speed : 0,
            total_cars: 0,
            max_time: 0,
            min_time: 0,
            average_speed: 0,
            close_calls: 0,
        }
    }

    pub fn update(&mut self, stats: &mut Stats, canvas: &mut Canvas<Window>, canvas_width: i32, canvas_height: i32, vehicle_width: i32, vehicle_height: i32, lanes: &mut Vec<Lane>) {
        // Initialize SDL2_image
        image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();

        // Load the background image
        let texture_creator = canvas.texture_creator();

        // Load the vehicle textures
        let vehicle_texture_up = texture_creator.load_texture("./assets/car-up.png").unwrap();
        let vehicle_texture_down = texture_creator.load_texture("./assets/car-down.png").unwrap();
        let vehicle_texture_left = texture_creator.load_texture("./assets/car-left.png").unwrap();
        let vehicle_texture_right = texture_creator.load_texture("./assets/voiture-de-course.png").unwrap();

        for vehicle in self.vehicles.iter_mut() {
            vehicle.move_forward(stats, lanes,canvas_width, canvas_height, vehicle_width);
            let rect = Rect::new(vehicle.position.x, vehicle.position.y, vehicle_width as u32, vehicle_height as u32);
            
            match vehicle.route {
                cars::Route::Down => canvas.copy_ex(&vehicle_texture_down, None, rect, vehicle.angle as f64, None, false, false).unwrap(),
                cars::Route::Up =>canvas.copy_ex(&vehicle_texture_up, None, rect, vehicle.angle as f64, None, false, false).unwrap(),
                cars::Route::Left =>{
                    let rect2 = Rect::new(vehicle.position.x, vehicle.position.y, vehicle_height as u32, vehicle_width as u32);
                    canvas.copy_ex(&vehicle_texture_left, None, rect2, vehicle.angle as f64, None, false, false).unwrap()
                },
                cars::Route::Right =>{
                    let rect2 = Rect::new(vehicle.position.x, vehicle.position.y, vehicle_height as u32, vehicle_width as u32);
                    canvas.copy_ex(&vehicle_texture_right, None, rect2, vehicle.angle as f64, None, false, false).unwrap()
                },            
            }
        }
    }

    pub fn has_collision(&mut self, new_vehicle: &Vehicle) -> (bool, Option<&mut Vehicle>) {
        for vehicle in self.vehicles.iter_mut() {
            let distance_x = (new_vehicle.position.x - vehicle.position.x).abs() as f32;
            let distance_y = (new_vehicle.position.y - vehicle.position.y).abs() as f32;

            if (distance_x < (vehicle.safe_distance as f32 *2.0)) && (distance_y < (vehicle.safe_distance as f32 *2.0)) {
                return (true, Some(vehicle));
            }
        }
        (false, None)
    }

}
