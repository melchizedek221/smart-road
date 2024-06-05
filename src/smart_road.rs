use std::time::{Duration, Instant};
use sdl2::render::WindowCanvas;

use super::{Intersection, Vehicle};

pub struct SmartRoad {
    pub intersection: Intersection,
    total_cars: u32,
    average_velocity: f32,
    last_add_time: Instant, 
    add_interval: Duration, 
}

impl SmartRoad {
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(),
            total_cars: 0,
            average_velocity: 0.0,
            last_add_time: Instant::now(), 
            add_interval: Duration::from_millis(500), 
        }
    }

    pub fn can_add_vehicle(&self) -> bool {
        self.last_add_time.elapsed() >= self.add_interval
    }

    pub fn add_vehicle(&mut self, vehicle: Vehicle, canvas: &mut WindowCanvas) {
        if self.can_add_vehicle() {
            self.intersection.add_vehicle(vehicle, canvas);
            self.total_cars += 1;
            self.last_add_time = Instant::now();
        }
    }

    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        self.intersection.waiting(canvas);
        if self.intersection.average_velocity() != 0.0 {
            self.average_velocity =
                (self.intersection.average_velocity() as f32 + self.average_velocity) / 2.0;
        }
        self.intersection.regulate(canvas);
        self.intersection.moves.drop_state();
    }

    pub fn stats(&self) -> Stats {
        let mut min_v = 0;
        if self.intersection.min_velocity != u32::MAX {
            min_v = self.intersection.min_velocity * 10;
        }
        let mut min_t = 0.0;
        if self.intersection.min_time != u32::MAX {
            min_t = self.intersection.min_time as f32 / 10.0;
        }

        Stats {
            total_cars: self.total_cars - self.intersection.waiting_room.len() as u32,
            nbr_passed: self.intersection.nbr_passed_vehicles,
            max_velocity: self.intersection.max_velocity * 10,
            min_velocity: min_v,
            average_velocity: self.average_velocity * 10.0,
            max_time: self.intersection.max_time as f32 / 10.0,
            min_time: min_t,
        }
    }
}


#[derive(Clone, Debug, Copy)]
pub struct Stats {
    pub total_cars: u32,
    pub nbr_passed: u32,
    pub max_velocity: u32,
    pub min_velocity: u32,
    pub average_velocity: f32,
    pub max_time: f32,
    pub min_time: f32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            total_cars: 0,
            nbr_passed: 0,
            max_velocity: 0,
            min_velocity: 0,
            average_velocity: 0.0,
            max_time: 0.0,
            min_time: 0.0,
        }
    }
}
