use crate::{Algorithm, Instruction, Managed, Moves};

use super::Vehicle;
use sdl2::render::WindowCanvas;
use std::collections::VecDeque;

pub struct Intersection {
    pub vehicles: VecDeque<Managed>,
    pub moves: Moves,
    pub waiting_room: Vec<Vehicle>,
    pub min_time: u32,
    pub max_time: u32,
    pub min_velocity: u32,
    pub max_velocity: u32,
    pub nbr_passed_vehicles: u32,
    pub avg_velocity: f32,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: VecDeque::new(),
            moves: Moves::new(),
            waiting_room: vec![],
            min_time: u32::MAX,
            max_time: u32::MIN,
            min_velocity: u32::MAX,
            max_velocity: u32::MIN,
            nbr_passed_vehicles: u32::MIN,
            avg_velocity: 0.0,
        }
    }
    pub fn waiting(&mut self, canvas: &mut WindowCanvas) {
        let list = self.waiting_room.clone();
        self.waiting_room = vec![];
        for v in list {
            self.add_vehicle(v, canvas);
        }
    }
    pub fn add_vehicle(&mut self, mut v: Vehicle, canvas: &mut WindowCanvas) {
        let instrs = self.instruct_vehicle(&v, canvas);
        v.time += 1;
        if instrs.len() == 0 {
            self.waiting_room.push(v);
            return;
        }
        self.vehicles.push_back(Managed::new(v, instrs));
    }

    pub fn instruct_vehicle(&mut self, v: &Vehicle, canvas: &mut WindowCanvas) -> VecDeque<Instruction> {
        let mut algo = Algorithm::new();
        let mut res = algo.algorithm(&self.moves, v, VecDeque::new(), canvas);
        if res.len() == 0 && self.moves.states.len() > 0 {
            return VecDeque::new();
        }
        let mut sim_v = v.clone();
        let mut ix = 0;

        while !sim_v.is_out() {
            let x = sim_v.position.x / 2;
            let y = sim_v.position.y / 2;
            if ix >= self.moves.states.len() {
                self.moves.add_state();
            }
            let (mut xs, mut ys) = (vec![x / 20], vec![y / 20]);
            if x % 20 != 0 {
                xs.push((x / 20) + 1);
            }
            if y % 20 != 0 {
                ys.push((y / 20) + 1);
            }
            for a in xs {
                for b in &ys {
                    self.moves.states[ix].occupy(a as usize, *b as usize);
                }
            }
            if ix >= res.len() {
                if sim_v.speed != super::Speed::High {
                    res.push_back(Instruction::Accelerate);
                    sim_v.accelerate();
                } else {
                    res.push_back(Instruction::Still);
                }
                sim_v.drive();
                ix += 1;
                continue;
            }
            match res[ix] {
                Instruction::Accelerate => sim_v.accelerate(),
                Instruction::Deaccelerate => sim_v.deaccelerate(),
                Instruction::Still => {}
            }
            sim_v.drive();
            ix += 1;
        }
        res
    }
    
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        let mut list = vec![];
        let mut total = 0.0;
        for ix in 0..self.vehicles.len() {
            let v = self.vehicles[ix].vehicle.get_speed();
            if v > self.max_velocity {
                self.max_velocity = v;
            }
            if v < self.min_velocity {
                self.min_velocity = v;
            }
            total += v as f32;
            self.vehicles[ix].vehicle.time += 1;
            self.vehicles[ix].follow_instruction(canvas);
            if self.vehicles[ix].is_empty_instructions() {
                self.nbr_passed_vehicles += 1;
                list.push(ix);
                let t = self.vehicles[ix].vehicle.time;
                if t > self.max_time {
                    self.max_time = t;
                }
                if t < self.min_time {
                    self.min_time = t;
                }
            }
        }
        if self.vehicles.len() > 0 {
            if self.average_velocity() == 0.0 {
                self.avg_velocity = total / self.vehicles.len() as f32;
            } else {
                self.avg_velocity =
                    ((total / self.vehicles.len() as f32) + self.avg_velocity) / 2.0;
            }
        }
        list.reverse();
        for jx in list {
            self.vehicles.remove(jx);
        }
    }
    
    pub fn average_velocity(&self) -> f32 {
        self.avg_velocity
    }
}


