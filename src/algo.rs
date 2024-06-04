use std::collections::{HashMap, VecDeque};

use crate::{Moves, Vehicle};


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
