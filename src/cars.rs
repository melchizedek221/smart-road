// use std::borrow::Borrow;
// use std::cell::RefCell;
// use std::process::id;
// use rand::thread_rng;
use rand::Rng;


use sdl2::keyboard::Keycode;
// use sdl2::rect::Rect;
// use sdl2::render::Canvas;
// use sdl2::video::Window;
use sdl2::{event::Event, rect::Point};

use crate::lane::Lane;

const VELO_1: i32 = 2;
const VELO_2: i32 = 4;
// const VELO_3: i32 = 12;
#[derive(Debug, Clone)]
pub struct Vehicle {
    id: usize,
    pub position: Point,
    pub route: Route,
    pub destination: Route,
    pub velocity: i32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
    pub time: f32,
    pub distance: f32,
    pub safe_distance: f32,
    pub lane: u32,
    pub angle: f32,
    // pub next_id: usize,

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Route {
    Up,
    Down,
    Left,
    Right,
}


impl Vehicle {
    pub fn new(route: Route, destination: Route, id:usize) -> Self {
        Self {
            id: id,
            position: Point::new(0, 0),
            destination,
            route,
            velocity: VELO_1,
            is_changed_direction: false,
            is_stopped: false,
            time: 0.0,
            distance: 0.0,
            safe_distance: 50.0,
            // lane: rand::thread_rng().gen_range(1..4), //1 left 2 middle 3 right
            angle: 0.0,
            lane: 2
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


    // fn is_too_close(&self, other: &Vehicle) -> bool {
    //     let x_dist = (self.position.x - other.position.x).abs();
    //     let y_dist = (self.position.y - other.position.y).abs();
    //     x_dist < 50 && y_dist < 120
    // }
    // fn distance_to_exit(&self, canvas_width: i32, canvas_height: i32) -> f32 {
    //     match self.route {
    //         Route::Up => {
    //             if self.angle == -90.0 {
    //                 self.position.x as f32
    //             } else {
    //                 self.position.y as f32
    //             }
    //         },
    //         Route::Down => {
    //             if self.angle == -90.0 {
    //                 (canvas_width - self.position.x) as f32
    //             } else {
    //                 (canvas_height - self.position.y) as f32
    //             }
    //         },
    //         Route::Left =>  {
    //             if self.angle == -90.0 {
    //                 (canvas_height - self.position.y) as f32
    //             } else {
    //                 self.position.x as f32
    //             }
    //         },
    //         Route::Right => {
    //             if self.angle == -90.0 {
    //                 self.position.y as f32
    //             } else {
    //                 (canvas_width - self.position.x) as f32
    //             }
    //         },
    //     }
    // }
    
    // fn is_in_intersection(&self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> bool {
    //     self.position.x >= x_min && self.position.x <= x_max && self.position.y >= y_min && self.position.y <= y_max
    // }

    pub fn move_forward(
        &mut self,
        lanes: &mut Vec<Lane>,
        canvas_width: i32,
        canvas_height: i32,
        vehicle_width: i32,
    ) {

        let all_vehicles: Vec<&mut Vehicle> = lanes
            .iter_mut()
            .flat_map(|lane| lane.vehicles.iter_mut())
            .filter(|vehicle| {
                vehicle.position.x <= canvas_width
                    && vehicle.position.x >= -vehicle_width
                    && vehicle.position.y >= -vehicle_width
                    && vehicle.position.y <= canvas_height && vehicle.id != self.id && vehicle.lane != 3 
                    && self.route != vehicle.route
            })
            .collect();

        // println!("all {:?}", all_vehicles);
        if self.is_in_zone_d() {
            println!("Yeeessssssssiiiir");
        }else {
            println!("Nooooooope");
        }

        println!("{}", self.position.y);

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
                    self.is_changed_direction = true;
                    self.destination = Route::Left;
                } else if self.lane == 3 && self.position.y < 485{
                    self.is_changed_direction = true;
                    self.destination = Route::Right;

                }

                // println!("self {:?}", self.position);
                for other_vehicle in all_vehicles {
                    if self.is_in_zone_a() {
                        match other_vehicle.route {
                            Route::Up => {},
                            Route::Right => {
                                if other_vehicle.is_in_int_a(){
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Down => {
                                if other_vehicle.is_in_int_a(){
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Left => {}
                        }
                    }
                }
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
                    self.is_changed_direction = true;
                    self.destination = Route::Right;
                } else if self.lane == 3 && self.position.y > 255 {
                    self.is_changed_direction = true;
                    self.destination = Route::Left;
                }
                
                for other_vehicle in all_vehicles {
                    if self.is_in_zone_d() {
                        match other_vehicle.route {
                            Route::Up => {
                                if other_vehicle.is_in_int_d(){
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Right => {},
                            Route::Down => {},
                            Route::Left => {
                                if other_vehicle.is_in_int_d() {
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            }
                        }
                    }
                }
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
                    self.is_changed_direction = true;
                    self.destination = Route::Down;
                } else if self.lane == 3 && self.position.x < 485 {
                    self.is_changed_direction = true;
                    self.destination = Route::Up;
                }
                
                for other_vehicle in all_vehicles {
                    if self.is_in_zone_b() {
                        match other_vehicle.route {
                            Route::Up => {
                                if other_vehicle.is_in_int_b() {
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Right => {
                                if other_vehicle.is_in_int_b() {
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Down => {},
                            Route::Left => {}
                        }
                    }
                }
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
                    self.is_changed_direction = true;
                    self.destination = Route::Up;

                } else if self.lane == 3 && self.position.x > 260 {
                    self.is_changed_direction = true;
                    self.destination = Route::Down;
                }
                // print!("x {} ", self.position.x);
                // if self.is_in_zone_c() {
                //     println!("yesssss");
                // }else{
                //     println!("noooooooooo");
                // }
                for other_vehicle in all_vehicles {
                    if self.is_in_zone_c(){
                        match other_vehicle.route {
                            Route::Up => {
                                if other_vehicle.is_in_int_a(){
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Right => {},
                            Route::Down => {
                                if other_vehicle.is_in_int_c(){
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            },
                            Route::Left => {
                                if other_vehicle.is_in_int_c() {
                                    self.velocity = 0;
                                }else {
                                    self.velocity = VELO_2;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_in_int_a(&mut self) -> bool {
        self.position.x > 350 && self.position.x < 530 && self.position.y < 530 +30 && self.position.y > 360 + 30
    }

    pub fn is_in_int_b(&mut self) -> bool {
        self.position.x > 365 && self.position.x < 530 && self.position.y < 380 +30 && self.position.y > 220 + 30
    }
    
    pub fn is_in_int_c(&mut self) -> bool {
        self.position.x > 221 && self.position.x < 395 && self.position.y < 530 +30 && self.position.y > 364 + 30
    }
    pub fn is_in_int_d(&mut self) -> bool {
        self.position.x > 221 && self.position.x < 395 && self.position.y < 380 +30 && self.position.y > 200 + 30
    }

    pub fn is_in_zone_a(&mut self) -> bool {
        self.position.y > 538 
    }

    pub fn is_in_zone_b(&mut self) -> bool {
        self.position.x > 542
    }

    pub fn is_in_zone_c(&mut self) -> bool {
        self.position.x < 395 
    }

    pub fn is_in_zone_d(&mut self) -> bool {
        self.position.y < 215
    }
}


pub fn handle_keyboard_event(
    event: &Event,
    lanes: &mut Vec<Lane>,
    canvas_width: i32,
    canvas_height: i32,
    vehicle_width: i32,
    id: usize,
) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Up, Route::Up, id);
            vehicle.spawn(Route::Up, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(3) {
                // lane.vehicles.push(vehicle);
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Down, Route::Down, id);
            vehicle.spawn(Route::Down, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(0) {
                // lane.vehicles.push(vehicle);
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Left, Route::Left, id);
            vehicle.spawn(Route::Left, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(2) {
                // lane.vehicles.push(vehicle);
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Right, Route::Right, id);
            vehicle.spawn(Route::Right, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(1) {
                // lane.vehicles.push(vehicle);
                if !lane.has_collision(&vehicle).0 {
                    lane.vehicles.push(vehicle);
                } 
            }
        }
        Event::KeyDown {
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
            let mut vehicle = Vehicle::new(random_route, Route::Down,id);
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
                // lane.vehicles.push(vehicle);
            }
        }
        _ => {}
    }
}