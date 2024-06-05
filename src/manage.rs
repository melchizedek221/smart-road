use std::collections::VecDeque;

use sdl2::render::WindowCanvas;

use crate::{Instruction, Vehicle};

#[derive(Clone, Debug)]
pub struct Managed {
    pub vehicle: Vehicle,
    pub instructions: VecDeque<Instruction>,
}

impl Managed {
    pub fn new(v: Vehicle, instrs: VecDeque<Instruction>) -> Self {
        Managed {
            vehicle: v,
            instructions: instrs,
        }
    }
    
    pub fn follow_instruction(&mut self, canvas: &mut WindowCanvas) {
        match self.instructions[0] {
            Instruction::Still => {}
            Instruction::Deaccelerate => self.vehicle.deaccelerate(),
            Instruction::Accelerate => self.vehicle.accelerate(),
        }
        self.vehicle.drive();
        self.vehicle.render(canvas);
        self.instructions.pop_front();
    }
    
    pub fn is_empty_instructions(&self) -> bool {
        self.instructions.len() == 0
    }
}