use rand::{Rand, Rng};
use sdl2::image::LoadTexture;
// use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

// vehicle
#[derive(Clone, Debug, Copy)]
pub struct Vehicle {
    pub position: Point,
    pub turn: Turning,
    pub direction: Direction,
    pub speed: Speed,
    pub time: u32,
    pub is_changed_direction: bool,
}

impl Vehicle {
    pub fn new(turn: Turning, direction: Direction, speed: Speed) -> Self {
        let (canvas_width, canvas_height, vehicle_width) = (800, 800, 25);
        let position = Vehicle::spawn(direction, turn, canvas_width, canvas_height, vehicle_width);

        Self {
            position,
            turn,
            direction,
            speed,
            time: 0,
            is_changed_direction: false,
        }
    }

    pub fn spawn(
        direction: Direction,
        turn: Turning,
        canvas_width: i32,
        canvas_height: i32,
        vehicle_width: i32,
    ) -> Point {
        let mut position = Point::new(0, 0);

        match direction {
            Direction::North => {
                match turn {
                    Turning::Left => {
                        position.x = (canvas_width / 2) + vehicle_width / 2 - 4;
                        position.y = canvas_height;
                    },
                    Turning::Straight => {
                        position.x = (canvas_width / 2) + vehicle_width / 2 + 40;
                        position.y = canvas_height;
                    },
                    Turning::Right => {
                        position.x = (canvas_width / 2) + vehicle_width / 2 + 84;
                        position.y = canvas_height;
                    }
                }
            }
            Direction::South => {
                // println!("doooooone");
                match turn {
                    Turning::Left => {
                        position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 + 4;
                        position.y = -vehicle_width;
                    },
                    Turning::Straight => {
                        position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 - 40;
                        position.y = -vehicle_width;
                    },
                    Turning::Right => {
                        position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2 - 84;
                        position.y = -vehicle_width;
                    }
                }
            }
            Direction::West => {
                match turn {
                    Turning::Left => {
                        position.x = canvas_width;
                        position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 + 4;
                    },
                    Turning::Straight => {
                        position.x = canvas_width;
                        position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 - 40;
                    },
                    Turning::Right => {
                        position.x = canvas_width;
                        position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2 - 84;
                    }
                }
            }
            Direction::East => {
                match turn {
                    Turning::Left => {
                        position.x = -vehicle_width;
                        position.y = canvas_height / 2 + vehicle_width / 2 - 4;
                    },
                    Turning::Straight => {
                        position.x = -vehicle_width;
                        position.y = canvas_height / 2 + vehicle_width / 2 + 40;
                    },
                    Turning::Right => {
                        position.x = -vehicle_width;
                        position.y = canvas_height / 2 + vehicle_width / 2 + 84;
                    }
                }
            }
        }

        position
    }

    pub fn accelerate(&mut self) {
        match self.speed {
            Speed::No => self.speed = Speed::Low,
            Speed::Low => self.speed = Speed::Normal,
            Speed::Normal => self.speed = Speed::High,
            Speed::High => {}
        }
    }

    pub fn deaccelerate(&mut self) {
        match self.speed {
            Speed::No => {}
            Speed::Low => self.speed = Speed::No,
            Speed::Normal => self.speed = Speed::Low,
            Speed::High => self.speed = Speed::Normal,
        }
    }


    pub fn drive(
        &mut self,
        // canvas: &mut WindowCanvas
    ) {
        match self.direction {
            Direction::North=> {
                if !self.is_changed_direction {
                    self.position.y -= self.speed as i32;
                } else {
                    let d = match self.turn {
                        Turning::Left => -1,
                        Turning::Straight => return,
                        Turning::Right => 1,
                    };
                    
                    self.position.x += d * self.speed as i32;
                }
                if self.turn == Turning::Left && self.position.y < 353 {
                    self.is_changed_direction = true;
                } else if self.turn == Turning::Right && self.position.y < 500{
                    self.is_changed_direction = true;
                }
            }
            
            Direction::South => {
                if !self.is_changed_direction {
                    self.position.y += self.speed as i32;
                } else {
                    let d = match self.turn {
                        Turning::Left => 1,
                        Turning::Straight => return,
                        Turning::Right => -1,
                    };
                    self.position.x += d * self.speed as i32;
                }
                if self.turn == Turning::Left && self.position.y > 392 {
                    self.is_changed_direction = true;
                } else if self.turn == Turning::Right && self.position.y > 260 {
                    self.is_changed_direction = true;
                }
            }

            Direction::West => {
                // println!("{:?}", self.turn);
                if !self.is_changed_direction {
                    self.position.x -= self.speed as i32;
                } else {
                    let d = match self.turn {
                        Turning::Left => 1,
                        Turning::Straight => return,
                        Turning::Right => -1,
                    };
                    self.position.y += d * self.speed as i32;
                }
                if self.turn == Turning::Left && self.position.x < 353 {
                    self.is_changed_direction = true;
                } else if self.turn == Turning::Right && self.position.x < 500 {
                    self.is_changed_direction = true;
                }
            }

            Direction::East => {
                if !self.is_changed_direction {
                    self.position.x += self.speed as i32;
                } else {
                    let d = match self.turn {
                        Turning::Left => -1,
                        Turning::Straight => return,
                        Turning::Right => 1,
                    };
                    self.position.y += d * self.speed as i32;
                }
                
                if self.turn == Turning::Left && self.position.x > 389 {
                    self.is_changed_direction = true;
                } else if self.turn == Turning::Right && self.position.x > 250 {
                    self.is_changed_direction = true;
                }
            }
        }
    }


    pub fn is_out(self) -> bool {
        self.position.x > 1000
            || self.position.x < -50
            || self.position.y > 1000
            || self.position.y < -50
    }
    
    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        let rect = Rect::new(self.position.x, self.position.y, 25, 50);
        let rect2 = Rect::new(self.position.x, self.position.y, 50, 25);

        let texture_creator = canvas.texture_creator();

        // Load the vehicle textures
        let vehicle_texture_up = texture_creator.load_texture("./assets/car-up.png").unwrap();
        let vehicle_texture_down = texture_creator.load_texture("./assets/car-down.png").unwrap();
        let vehicle_texture_left = texture_creator.load_texture("./assets/car-left.png").unwrap();
        let vehicle_texture_right = texture_creator.load_texture("./assets/car-right.png").unwrap();

        // Default angle is 0
        let mut angle = 0.0;

        // Only change angle if the direction has changed
        if self.is_changed_direction {
            angle = match self.turn {
                Turning::Left => -90.0,
                Turning::Right => 90.0,
                Turning::Straight => 0.0,
            };
        }

        match self.direction {
            Direction::South => canvas.copy_ex(&vehicle_texture_down, None, rect, angle, None, false, false).unwrap(),
            Direction::North => canvas.copy_ex(&vehicle_texture_up, None, rect, angle, None, false, false).unwrap(),
            Direction::East => canvas.copy_ex(&vehicle_texture_right, None, rect2, angle, None, false, false).unwrap(),
            Direction::West => canvas.copy_ex(&vehicle_texture_left, None, rect2, angle, None, false, false).unwrap(),
        }
    }
    
    pub fn get_speed(self) -> u32 {
        match self.speed {
            Speed::High => 30,
            Speed::Normal => 20,
            Speed::Low => 10,
            Speed::No => 0,
        }
    }
}

// turning

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Turning {
    Left,
    Right,
    Straight,
}

impl Rand for Turning {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Turning::Left,
            1 => Turning::Right,
            _ => Turning::Straight,
        }
    }
}

// direction

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }
}

//speed
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Speed {
    No = 0,
    Low = 7,
    Normal = 15,
    High = 25,
}

impl Rand for Speed {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Speed::Low,
            1 => Speed::Normal,
            _ => Speed::High,
        }
    }
}