pub enum Direction {
    North,
    East,
    West,
    South,
}

pub struct Vehicle {
    pub direction: Direction,
    pub velocity: i32,
    pub x: i32,
    pub y: i32,
    pub turn_counter: i32,
}

impl Vehicle {
    pub fn new(direction: Direction, velocity: i32, x: i32, y: i32,turn_counter: i32,) -> Self {
        Self {
            direction,
            velocity,
            x,
            y,
            turn_counter
        }
    }

    pub fn move_car(&mut self){
        match self.direction {
            Direction::North => {
                self.y -= 1; 
                // self.x = 300;
            }
            Direction::South => {
                self.y += 1;
            }
            Direction::East => {
                self.x += 1; 
            }
            Direction::West => {
                self.x -= 1;
            }
        }
    }


}
