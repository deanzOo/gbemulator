use std::collections::HashMap;
use crate::{CPU};
use crate::instructions_loader::Instruction;
use crate::lr35902_instructions_impl::load_lr35902_instructions;

// instruction executor function pointer type, args: cpu
type InstructionExecutor = fn(&mut CPU);

pub struct InstructionImpl {
    pub(crate) instruction: Instruction,
    pub(crate) executor: InstructionExecutor
}

impl InstructionImpl {
    pub(crate) fn new(instruction: Instruction, executor: InstructionExecutor) -> Self {
        Self {
            instruction,
            executor
        }
    }
}

pub fn load_instructions(cpu_type: String) -> (HashMap<u8, InstructionImpl>, HashMap<u8, InstructionImpl>) {
    if cpu_type == "LR35902" {
        return load_lr35902_instructions();
    } else {
        panic!("Invalid CPU type");
    }
}