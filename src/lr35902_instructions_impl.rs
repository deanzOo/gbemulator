use std::collections::HashMap;
use crate::cpu::CPU;
use crate::instruction_factory::InstructionImpl;
use crate::instructions_loader::instructions_from_json;

pub fn load_lr35902_instructions() -> (HashMap<u8, InstructionImpl>, HashMap<u8, InstructionImpl>) {
    let mut instruction_set = instructions_from_json(String::from("D:\\Dev\\gbemulator\\src\\LR35902_instructions.json"));
    // let mut cb_instruction_set = instructions_from_json(String::from("LR35902_cb_instructions.json"));

    let mut instruction_impls: HashMap<u8, InstructionImpl> = HashMap::new();
    let mut cb_instruction_impls: HashMap<u8, InstructionImpl> = HashMap::new();

    instruction_impls.insert(0x00, InstructionImpl::new(instruction_set.remove(&0x00).unwrap(), |cpu: &mut CPU| {}));
    instruction_impls.insert(0x01, InstructionImpl::new(instruction_set.remove(&0x01).unwrap(), |cpu: &mut CPU| {
        let value = cpu.read_word_at_pc();
        cpu.set_bc(value);
    }));
    instruction_impls.insert(0x02, InstructionImpl::new(instruction_set.remove(&0x02).unwrap(), |cpu: &mut CPU| {
        let value = cpu.get_a();
        let address = cpu.get_bc();
        cpu.write_byte(address, value);
    }));

    instruction_impls.insert(0x31, InstructionImpl::new(instruction_set.remove(&0x31).unwrap(), |cpu: &mut CPU| {
        let value = cpu.read_word_at_pc();
        cpu.set_sp(value);
    }));

    instruction_impls.insert(0xD3, InstructionImpl::new(instruction_set.remove(&0x03).unwrap(), |cpu: &mut CPU| {
        panic!("Invalid instruction");
    }));

    instruction_impls.insert(0xC3, InstructionImpl::new(instruction_set.remove(&0xC3).unwrap(), |cpu: &mut CPU| {
        let address = cpu.read_word_at_pc();
        cpu.set_pc(address);
    }));

    instruction_impls.insert(0xEA, InstructionImpl::new(instruction_set.remove(&0xEA).unwrap(), |cpu: &mut CPU| {
        let val = cpu.get_a();
        let address = cpu.read_word_at_pc();
        cpu.write_byte(address, val);
    }));

    instruction_impls.insert(0xF3, InstructionImpl::new(instruction_set.remove(&0xF3).unwrap(), |cpu: &mut CPU| {
        cpu.set_ime(false);
    }));

    (instruction_impls, cb_instruction_impls)
}