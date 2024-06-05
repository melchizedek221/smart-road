use std::collections::{HashMap, VecDeque};
use sdl2::render::WindowCanvas;
use crate::{Moves, Vehicle, Direction, Speed};

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

    fn generate_key(&self, instr: &VecDeque<Instruction>) -> String {
        let mut algo = (0, 0, 0);
        for s in instr {
            match *s {
                Instruction::Accelerate => algo.0 += 1,
                Instruction::Still => algo.1 += 1,
                Instruction::Deaccelerate => algo.2 += 1,
            }
        }
        format!("{}:{}:{}", algo.0, algo.1, algo.2)
    }

    fn check_occupied(&self, x: i32, y: i32, a1: i32, b1: i32, sim_moves: &mut Moves) -> bool {
        let (mut xs, mut ys) = (vec![x / 20], vec![y / 20]);
        if x % 20 != 0 {
            xs.push((x / 20) + 1);
        }
        if y % 20 != 0 {
            ys.push((y / 20) + 1);
        }
        for a in xs {
            for b in &ys {
                if sim_moves.states[0].is_occupied(a as usize, *b as usize)
                    || sim_moves.states[0].is_occupied((a + a1) as usize, (*b + b1) as usize) {
                    return true;
                }
            }
        }
        false
    }

    fn execute_and_check(
        &mut self,
        moves: &Moves,
        v: &Vehicle,
        instr: &VecDeque<Instruction>,
        action: Instruction,
        canvas: &mut WindowCanvas
    ) -> VecDeque<Instruction> {
        let mut sim_v = v.clone();
        let mut m = moves.clone();
        let mut instr_clone = instr.clone();
        
        match action {
            Instruction::Accelerate => sim_v.accelerate(),
            Instruction::Deaccelerate => sim_v.deaccelerate(),
            _ => {}
        }
        
        sim_v.drive();
        m.drop_state();
        instr_clone.push_back(action);
        
        self.algorithm(&m, &sim_v, instr_clone, canvas)
    }

    pub fn algorithm(
        &mut self,
        moves: &Moves,
        v: &Vehicle,
        instr: VecDeque<Instruction>,
        canvas: &mut WindowCanvas
    ) -> VecDeque<Instruction> {
        if v.is_out() || moves.states.is_empty() {
            return instr;
        }

        let key = self.generate_key(&instr);
        if let Some(cached_res) = self.visited.get(&key) {
            return cached_res.clone();
        }

        let (x, y) = (v.position.x / 2, v.position.y / 2);
        let (a1, b1) = match v.direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };

        if self.check_occupied(x, y, a1, b1, &mut moves.clone()) {
            return VecDeque::new();
        }

        if v.speed != Speed::High {
            let res = self.execute_and_check(moves, v, &instr, Instruction::Accelerate, canvas);
            if !res.is_empty() {
                self.visited.insert(key, res.clone());
                return res;
            }
        }

        let res = self.execute_and_check(moves, v, &instr, Instruction::Still, canvas);
        if !res.is_empty() {
            self.visited.insert(key, res.clone());
            return res;
        }

        if v.speed != Speed::No {
            let res = self.execute_and_check(moves, v, &instr, Instruction::Deaccelerate, canvas);
            if !res.is_empty() {
                self.visited.insert(key, res.clone());
                return res;
            }
        }

        VecDeque::new()
    }
}
