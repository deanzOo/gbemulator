mod rom;
mod bus;
mod cpu;

mod instructions_loader;
mod instruction_factory;
mod lr35902_instructions_impl;

use crate::cpu::CPU;

fn main() {
    let rom_filepath = String::from("roms/cpu_instrs.gb");
    let cpu_type = String::from("LR35902");

    let mut cpu = CPU::new(rom_filepath, cpu_type);

    cpu.run();
}
