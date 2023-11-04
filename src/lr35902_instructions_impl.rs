use std::collections::HashMap;
use crate::cpu::CPU;
use crate::instruction_factory::InstructionImpl;
use crate::instructions_loader::{instructions_from_yaml};

pub fn load_lr35902_instructions() -> (HashMap<u8, InstructionImpl>, HashMap<u8, InstructionImpl>) {
    let mut instruction_set = instructions_from_yaml(String::from("D:\\Dev\\gbemulator\\src\\LR35902_instructions.yaml"));
    // let mut cb_instruction_set = instructions_from_json(String::from("LR35902_cb_instructions.yaml"));

    let mut instruction_impls: HashMap<u8, InstructionImpl> = HashMap::new();
    let mut cb_instruction_impls: HashMap<u8, InstructionImpl> = HashMap::new();

    instruction_impls.insert(0x00, InstructionImpl::new(instruction_set.remove(&0x00).unwrap(), |cpu: &mut CPU| {}));
    instruction_impls.insert(0xC3, InstructionImpl::new(instruction_set.remove(&0xC3).unwrap(), |cpu: &mut CPU| {
        if cpu.get_z_flag() {
            let val = cpu.read_word_at_pc();
            cpu.set_pc(val);
        }
    }));

    (instruction_impls, cb_instruction_impls)
}