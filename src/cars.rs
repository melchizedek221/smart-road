// use std::time::Duration;
use std::time::Instant;
// use rand::thread_rng;
use rand::Rng;

// const TIMEOUT_DURATION: Duration = Duration::from_millis(100); // 5 seconds timeout
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, rect::Point};

use crate::lane::Lane;
use crate::statistics::Stats;
// use crate::statistics::Stats;

const VELO_1: i32 = 2;
const VELO_2: i32 = 5;
const VELO_3: i32 = 10;
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: usize,
    pub position: Point,
    pub route: Route,
    pub destination: Route,
    pub velocity: i32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
    pub time: Instant,
    pub distance: f32,
    pub safe_distance: f64,
    pub lane: u32,
    pub angle: f32,
    pub waiting: bool,
    pub statut: bool,
    pub entry_time: Instant,
    pub exit_time: Option<Instant>,
    pub duration: f64

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Route {
    Up,
    Down,
    Left,
    Right,
}


impl Vehicle {
    pub fn new(route: Route, _velocity: i32, destination: Route, id:usize) -> Self {
        // let destination = Self::random(route);
        
        Self {
            id: id,
            position: Point::new(0, 0),
            destination,
            route,
            velocity: VELO_1,
            is_changed_direction: false,
            is_stopped: false,
            time: Instant::now(),
            distance: 0.0,
            safe_distance: 43.0,
            lane: rand::thread_rng().gen_range(1..4), //1 left 2 middle 3 right
            angle: 0.0,
            waiting: false,
            statut: false,
            entry_time: Instant::now(),
            exit_time: None,
            duration: 0.0,
            // lane: 2,
        }
    }

    pub fn spawn(
        &mut self,
        direction: Route,
        canvas_width: i32,
        canvas_height: i32,
        vehicle_width: i32,
    ) {
        match direction {
            Route::Up => {
                match self.lane {
                    1 => {
                        self.position.x = (canvas_width / 2) + vehicle_width / 2 - 4;
                        self.position.y = canvas_height;
                    },
                    2 => {
                        self.position.x = (canvas_width / 2) + vehicle_width / 2 + 40;
                        self.position.y = canvas_height;
                    },
                    3 => {
                        self.position.x = (canvas_width / 2) + vehicle_width / 2 + 84;
                        self.position.y = canvas_height;
                    }
                    _ => panic!("Invalid direction")
                }
            }
            Route::Down => {
                match self.lane {
                    1 => {
                        self.position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 + 4;
                        self.position.y = -vehicle_width;
                    },
                    2 => {
                        self.position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 - 40;
                        self.position.y = -vehicle_width;
                    },
                    3 => {
                        self.position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 - 84;
                        self.position.y = -vehicle_width;
                    }
                    _ => panic!("Invalid direction")
                }

            }
            Route::Left => {
                match self.lane {
                    1 => {
                        self.position.x = canvas_width;
                        self.position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 + 4;
                    },
                    2 => {
                        self.position.x = canvas_width;
                        self.position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 - 40;
                    },
                    3 => {
                        self.position.x = canvas_width;
                        self.position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 - 84;
                    }
                    _ => panic!("Invalid direction")
                }
            }

            Route::Right => {
                match self.lane {
                    1 => {
                        self.position.x = -vehicle_width;
                        self.position.y = canvas_height / 2 + vehicle_width / 2 - 4;
                    },
                    2 => {
                        self.position.x = -vehicle_width;
                        self.position.y = canvas_height / 2 + vehicle_width / 2 + 40;
                    },
                    3 => {
                        self.position.x = -vehicle_width;
                        self.position.y = canvas_height / 2 + vehicle_width / 2 + 84;
                    }
                    _ => panic!("Invalid direction")
                }

            }
        }
    }

    fn distance_to_exit(&self, canvas_width: i32, canvas_height: i32) -> f32 {
        match self.route {
            Route::Up => {
                if self.angle == -90.0 {
                    self.position.x as f32
                } else {
                    self.position.y as f32
                }
            },
            Route::Down => {
                if self.angle == -90.0 {
                    (canvas_width - self.position.x) as f32
                } else {
                    (canvas_height - self.position.y) as f32
                }
            },
            Route::Left =>  {
                if self.angle == -90.0 {
                    (canvas_height - self.position.y) as f32
                } else {
                    self.position.x as f32
                }
            },
            Route::Right => {
                if self.angle == -90.0 {
                    self.position.y as f32
                } else {
                    (canvas_width - self.position.x) as f32
                }
            },
        }
    }
    
    fn potential_colliders<'a>(&self, cars: &'a mut Vec<&mut Vehicle>) -> Vec<&'a mut Vehicle> {
        let mut vec: Vec<&mut Vehicle> = Vec::new();
        match self.route {
            Route::Up => {
                match self.lane {
                    1 => {
                        for car in cars.iter_mut(){
                            if ((car.route == Route::Right && car.lane == 1) || (car.route == Route::Right && car.lane == 2) ||
                            (car.route == Route::Down && car.lane == 1) || (car.route == Route::Down && car.lane == 2) ||
                            (car.route == Route::Left && car.lane == 1)) && !car.is_out()
                            {
                                vec.push(car);
                            }
                        }
                        
                    },
                    2 => {
                        for car in cars.iter_mut(){
                            if ((car.route == Route::Down && car.lane == 1) || (car.route == Route::Right && car.lane == 2) ||
                            (car.route == Route::Left && car.lane == 1) || (car.route == Route::Left && car.lane == 2))  && !car.is_out()
                            {
                                vec.push(car);
                            }
                        }
                    },
                    _ => print!(""),
                }

            },
            Route::Down => {
                match self.lane {
                    1 => {
                        for car in cars.iter_mut(){
                            if ((car.route == Route::Up && car.lane == 1) || (car.route == Route::Left && car.lane == 1) 
                            || (car.route == Route::Right && car.lane == 1) || (car.route == Route::Left && car.lane == 2)
                            || (car.route == Route::Up && car.lane == 2)) && !car.is_out()
                            
                            {
                                vec.push(car);
                            }
                        }

                    },
                    2 => {
                        for car in cars.iter_mut(){
                            if 
                            ((car.route == Route::Up && car.lane == 1) || (car.route == Route::Right && car.lane == 2) 
                            || (car.route == Route::Left && car.lane == 2)
                            || (car.route == Route::Right && car.lane == 1)) && !car.is_out()
                            {
                                vec.push(car);
                            }
                        }
                    },
                    _ => println!(""),
                }

            },
            Route::Right => {
                match self.lane {
                    1 => {
                        for car in cars.iter_mut(){
                            if ((car.route == Route::Up && car.lane == 1) || (car.route == Route::Down && car.lane == 2) 
                            || (car.route == Route::Down && car.lane == 1) || (car.route == Route::Left && car.lane == 2)
                            || (car.route == Route::Left && car.lane == 1)) && !car.is_out()
                            {
                                vec.push(car);
                            }
                        }

                    },
                    2 => {
                        for car in cars.iter_mut(){
                            if 
                            ((car.route == Route::Up && car.lane == 1) || (car.route == Route::Down && car.lane == 2) 
                            || (car.route == Route::Up && car.lane == 2) || (car.route == Route::Left && car.lane == 1)) && !car.is_out()
                            
                            {
                                vec.push(car);
                            }
                        }
                    },
                    _ => print!(""),
                }
            },
            _ => {
                match self.lane {
                    1 => {
                        for car in cars.iter_mut(){
                            if ((car.route == Route::Up && car.lane == 1) || (car.route == Route::Up && car.lane == 2) 
                            || (car.route == Route::Right && car.lane == 1) || (car.route == Route::Right && car.lane == 2)
                            || (car.route == Route::Down && car.lane == 1)) && !car.is_out()
                            {
                                vec.push(car);
                            }
                        }

                    },
                    2 => {
                        for car in cars.iter_mut(){
                            if 
                            ((car.route == Route::Up && car.lane == 2) || (car.route == Route::Down && car.lane == 2) 
                            || (car.route == Route::Down && car.lane == 1) || (car.route == Route::Right && car.lane == 1)) && !car.is_out()
                            
                            {
                                vec.push(car);
                            }
                        }
                    },
                    _ => print!(""),
                }
            },
        }
        vec

    }

    fn is_out(&self) -> bool {
        match self.route {
            Route::Up => {
                if self.lane == 1 {
                    return self.position.x < 240;
                } else if self.lane == 2 {
                    return  self.position.y < 224;
                } else {
                    return self.position.x > 504;
                }
            },
            Route::Down => {
                if self.lane == 1 {
                    return self.position.x > 535;
                } else if self.lane == 2 {
                    return  self.position.y > 514;
                } else {
                    return self.position.x < 248;
                }
            },
            Route::Left => {
                if self.lane == 1 {
                    return self.position.y > 528;
                } else if self.lane == 2 {
                    return  self.position.x < 225;
                } else {
                    return self.position.y < 232;
                }
            },
            _=> {
                if self.lane == 1 {
                    return self.position.y < 241;
                } else if self.lane == 2 {
                    return  self.position.x > 521;
                } else {
                    return self.position.y > 533;
                }
            }
            
        }
        
    }

    fn is_out_of_canv(&self) -> bool {
        (self.position.x >= 800 && self.position.x <= 805)
        || (self.position.x <= -25 && self.position.x >= -30)
        || (self.position.y <= -25 && self.position.y >= -30)
        || (self.position.y >= 800 && self.position.y <= 805)
    }

    pub fn move_forward(
        &mut self,
        stats: &mut Stats,
        lanes: &mut Vec<Lane>,
        canvas_width: i32,
        canvas_height: i32,
        vehicle_width: i32,
    ) {

        // println!("here {:?}", self.position);
        if self.is_out_of_canv(){
            self.exit_time = Some(Instant::now());
            self.duration = self.get_duration_in_seconds();
            // vehicles.push(self)
            // println!("yessss");
            if self.duration > 1.0 {
                stats.durations.push(self.duration);
            }
            stats.velocities.push(self.velocity as f64)
        }

        // ANOTHER TEST
        let mut binding = lanes.clone();
        let mut all_vehicles: Vec<&mut Vehicle> = binding
            .iter_mut()
            .flat_map(|lane| lane.vehicles.iter_mut())
            .filter(|vehicle| {
                vehicle.position.x <= canvas_width
                    && vehicle.position.x >= -vehicle_width
                    && vehicle.position.y >= -vehicle_width
                    && vehicle.position.y <= canvas_height
            })
            .collect();

        match self.route {
            Route::Up => {
                if !self.is_changed_direction {
                    self.position.y -= self.velocity;
                } else {
                    let d = match self.lane {
                        1 => -1,
                        2 => return,
                        3 => 1,
                        _ => panic!("Invalid lane"),
                    };
                    self.angle = match self.lane {
                        1 => -90.0,
                        2 => 0.0,
                        3 => 90.0,
                        _ => panic!(),
                    };
                    self.position.x += d * self.velocity;
                }

                if self.lane == 1 && self.position.y < 353 {
                    self.is_changed_direction = true
                } else if self.lane == 3 && self.position.y < 485{
                    self.is_changed_direction = true
                }

                let cars = self.potential_colliders(&mut all_vehicles);
                let len_cars = cars.len();
                for car_bis in cars {
                    let self_distance = self.distance_to_exit(canvas_width, canvas_height);
                    let other_distance = car_bis.distance_to_exit(canvas_width, canvas_height);
                    if self_distance < other_distance {
                        self.waiting = false
                    } else {
                        self.waiting = true;
                        self.time = Instant::now();
                        break;
                    }
                }
                if self.waiting {
                    self.velocity = 1;
                }else {
                    self.velocity = VELO_3;
                }
                if len_cars == 0{
                    self.velocity = VELO_2;
                }
                if self.is_out() {
                    self.velocity = VELO_2;
                }
                
                // if self.position.y <= 508 && self.position.y >= 507{
                //     stats.vehicpass += 1;
                // }
                stats.velocities.push(self.velocity as f64)
            }
            Route::Down => {
                if !self.is_changed_direction {
                    self.position.y += self.velocity;
                } else {
                    let d = match self.lane {
                        1 => 1,
                        2 => return,
                        3 => -1,
                        _ => panic!("Invalid lane"),
                    };
                    self.angle = match self.lane {
                        1 => -90.0,
                        2 => 0.0,
                        3 => 90.0,
                        _ => panic!(),
                    };
                    self.position.x += d * self.velocity;
                }

                if self.lane == 1 && self.position.y > 389 {
                    self.is_changed_direction = true
                } else if self.lane == 3 && self.position.y > 255 {
                    self.is_changed_direction = true
                }

                let cars = self.potential_colliders(&mut all_vehicles);
                let len_cars = cars.len();
                for car_bis in cars {
                    let self_distance = self.distance_to_exit(canvas_width, canvas_height);
                    let other_distance = car_bis.distance_to_exit(canvas_width, canvas_height);
                    if self_distance < other_distance {
                        self.waiting = false

                    } else {
                        self.waiting = true;
                        break;
                    }
                }
                if self.waiting {
                    self.velocity = 1
                }else {
                    self.velocity = VELO_3;
                }
                if len_cars == 0{
                    self.velocity = VELO_2;
                }
                if self.is_out() {
                    self.velocity = VELO_2;
                }
                               
                // if self.position.y >= 242 && self.position.y <= 243{
                //     stats.vehicpass += 1;
                // }
                stats.velocities.push(self.velocity as f64)
            }
            Route::Left => {
                if !self.is_changed_direction {
                    self.position.x -= self.velocity;
                } else {
                    let d = match self.lane {
                        1 => 1,
                        2 => return,
                        3 => -1,
                        _ => panic!("Invalid lane"),
                    };
                    self.angle = match self.lane {
                        1 => -90.0,
                        2 => 0.0,
                        3 => 90.0,
                        _ => panic!(),
                    };
                    self.position.y += d * self.velocity;
                }

                if self.lane == 1 && self.position.x < 353 {
                    self.is_changed_direction = true
                } else if self.lane == 3 && self.position.x < 485 {
                    self.is_changed_direction = true
                }

                let cars = self.potential_colliders(&mut all_vehicles);
                let len_cars = cars.len();

                for car_bis in cars {
                    let self_distance = self.distance_to_exit(canvas_width, canvas_height);
                    let other_distance = car_bis.distance_to_exit(canvas_width, canvas_height);
                    if self_distance < other_distance {
                        self.waiting = false

                    } else {
                        self.waiting = true;
                        self.time = Instant::now();
                        break;
                    }
                    
                }
                if self.waiting {
                    self.velocity = 1
                }else {
                    self.velocity = VELO_3;
                }
                if len_cars == 0{
                    self.velocity = VELO_2;
                }
                if self.is_out() {
                    self.velocity = VELO_2;
                }
                
                // if self.position.x >= 508 && self.position.x <= 509{
                //     stats.vehicpass += 1;
                // }
                stats.velocities.push(self.velocity as f64)
            }
            Route::Right => {
                if !self.is_changed_direction {
                    self.position.x += self.velocity;
                } else {
                    let d = match self.lane {
                        1 => -1,
                        2 => return,
                        3 => 1,
                        _ => panic!("Invalid lane"),
                    };
                    self.angle = match self.lane {
                        1 => -90.0,
                        2 => 0.0,
                        3 => 90.0,
                        _ => panic!(),
                    };
                    self.position.y += d * self.velocity;
                }
                if self.lane == 1 && self.position.x > 392 {
                    self.is_changed_direction = true
                } else if self.lane == 3 && self.position.x > 260 {
                    self.is_changed_direction = true
                }

                
                let cars = self.potential_colliders(&mut all_vehicles);
                let len_cars = cars.len();

                for car_bis in cars {
                    let self_distance = self.distance_to_exit(canvas_width, canvas_height);
                    let other_distance = car_bis.distance_to_exit(canvas_width, canvas_height);
                    if self_distance < other_distance {
                        self.waiting = false
                    } else {
                        self.waiting = true;
                        self.time = Instant::now();
                        break;
                    }
                }
                
                if self.waiting {
                    self.velocity = 1
                }else {
                    self.velocity = VELO_3;
                }
                if len_cars == 0{
                    self.velocity = VELO_2;
                }
                if self.is_out() {
                    self.velocity = VELO_2;
                }
                // println!("ve {:?}", self);
                
                // if self.position.x >= 242 && self.position.x <= 243{
                //     stats.vehicpass += 1;
                // }
                stats.velocities.push(self.velocity as f64)
            }
        }

        //close calls
        for other in all_vehicles.iter() {
            if self.id != other.id {
                let dist = (((self.position.x - other.position.x).pow(2)
                    + (self.position.y - other.position.y).pow(2)) as f64)
                .sqrt() as f64;
                if dist < self.safe_distance {
                    stats.close_calls += 1;
                }
            }
        }

        stats.update_stats();

        self.check_and_increment_pass_count(stats);
    }

    fn check_and_increment_pass_count(&self, stats: &mut Stats) {
        match self.route {
            Route::Up => {
                if self.position.y <= 508 && self.position.y >= 507 {
                    stats.vehicpass += 1;
                }
            },
            Route::Down => {
                if self.position.y >= 242 && self.position.y <= 243 {
                    stats.vehicpass += 1;
                }
            },
            Route::Left => {
                if self.position.x >= 508 && self.position.x <= 509 {
                    stats.vehicpass += 1;
                }
            },
            Route::Right => {
                if self.position.x >= 242 && self.position.x <= 243 {
                    stats.vehicpass += 1;
                }
            },
        }
    }

    pub fn get_duration_in_seconds(&self) -> f64 {
        let duration = self.exit_time.unwrap().duration_since(self.entry_time);
        duration.as_secs_f64()
    }
}


pub fn handle_keyboard_event(
    event: &Event,
    lanes: &mut Vec<Lane>,
    canvas_width: i32,
    canvas_height: i32,
    vehicle_width: i32,
    id: usize
) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
        
            let mut vehicle = Vehicle::new(Route::Up, 1, Route::Left,id);
            vehicle.spawn(Route::Up, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(3) {
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Down, 1, Route::Down, id);
            vehicle.spawn(Route::Down, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(0) {
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Left, 1, Route::Left, id);
            vehicle.spawn(Route::Left, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(2) {
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Right, 1, Route::Right, id);
            vehicle.spawn(Route::Right, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(1) {
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyUp {
            keycode: Some(Keycode::R),
            ..
        } => {
            let mut rng = rand::thread_rng();
            let random_route = match rng.gen_range(0..4) {
                0 => Route::Up,
                1 => Route::Down,
                2 => Route::Left,
                _ => Route::Right,
            };
            let mut vehicle = Vehicle::new(random_route, 1, Route::Down,id);
            vehicle.spawn(random_route, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = match random_route {
                Route::Up => lanes.iter_mut().nth(3),
                Route::Down => lanes.iter_mut().nth(0),
                Route::Left => lanes.iter_mut().nth(2),
                Route::Right => lanes.iter_mut().nth(1),
            } {
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        _ => {}
    }
}
