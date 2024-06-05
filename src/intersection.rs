use crate::{Managed, Moves};

use super::Vehicle;
use sdl2::render::WindowCanvas;
use std::collections::{HashMap, VecDeque};

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

    pub fn instruct_vehicle(&mut self, v: &Vehicle, _canvas: &mut WindowCanvas) -> VecDeque<Instruction> {
        let mut algo = Algorithm::new();
        let mut res = algo.algorithm(&self.moves, v, VecDeque::new());
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


//algo
#[derive(Clone, Debug)]
pub enum Instruction {
    Deaccelerate,
    Still,
    Accelerate,
}

pub struct Algorithm {
    visited: HashMap<String, VecDeque<Instruction>>,
}

impl Algorithm {
    pub fn new() -> Self {
        Algorithm {
            visited: HashMap::new(),
        }
    }

    pub fn algorithm(
        &mut self,
        moves: &Moves,
        v: &Vehicle,
        instr: VecDeque<Instruction>,
    ) -> VecDeque<Instruction> {
        if v.is_out() || moves.states.len() == 0 {
            return instr;
        }
        let mut algo = (0, 0, 0);
        for s in &instr {
            match *s {
                Instruction::Accelerate => algo.0 += 1,
                Instruction::Still => algo.1 += 1,
                Instruction::Deaccelerate => algo.2 += 1,
            }
        }
        let key = format!("{}:{}:{}", algo.0, algo.1, algo.2);
        if self.visited.contains_key(&key) {
            let mut res = VecDeque::new();
            match self.visited.get(&key) {
                Some(v) => res = v.clone(),
                None => {}
            }
            return res;
        }
        let x = v.position.x / 2;
        let y = v.position.y / 2;
        let (mut xs, mut ys) = (vec![x / 20], vec![y / 20]);
        let mut sim_moves = moves.clone();
        if x % 20 != 0 {
            xs.push((x / 20) + 1);
        }
        if y % 20 != 0 {
            ys.push((y / 20) + 1);
        }
        let (mut a1, mut b1) = (0, 0);
        match v.direction {
            super::Direction::North => b1 -= 1,
            super::Direction::South => b1 += 1,
            super::Direction::East => a1 += 1,
            super::Direction::West => a1 -= 1,
        }
        for a in xs {
            for b in &ys {
                let mut ok = sim_moves.states[0].is_occupied(a as usize, *b as usize);
                if ok {
                    return VecDeque::new();
                }
                ok = sim_moves.states[0].is_occupied((a + a1) as usize, (*b + b1) as usize);
                if ok {
                    return VecDeque::new();
                }
            }
        }
        let mut sim_v1 = v.clone();
        let mut m1 = moves.clone();
        let mut instr1 = instr.clone();
        let mut res: VecDeque<Instruction>;
        if v.speed != super::Speed::High {
            sim_v1.accelerate();
            sim_v1.drive();
            m1.drop_state();
            instr1.push_back(Instruction::Accelerate);
            res = self.algorithm(&m1, &sim_v1, instr1);
            if res.len() > 0 {
                self.visited.insert(key, res.clone());
                return res;
            }
        }
        sim_v1 = v.clone();
        m1 = moves.clone();
        instr1 = instr.clone();
        sim_v1.drive();
        m1.drop_state();
        instr1.push_back(Instruction::Still);
        res = self.algorithm(&m1, &sim_v1, instr1);
        if res.len() > 0 {
            self.visited.insert(key, res.clone());
            return res;
        }
        if v.speed != super::Speed::No {
            sim_v1 = v.clone();
            m1 = moves.clone();
            instr1 = instr.clone();
            sim_v1.deaccelerate();
            sim_v1.drive();
            m1.drop_state();
            instr1.push_back(Instruction::Deaccelerate);
            res = self.algorithm(&m1, &sim_v1, instr1);
            if res.len() > 0 {
                self.visited.insert(key, res.clone());
                return res;
            }
        }
        self.visited.insert(key, res.clone());
        res
    }
}
