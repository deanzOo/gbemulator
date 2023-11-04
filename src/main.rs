mod rom;
mod bus;
mod cpu;

mod instructions_loader;
mod instruction_impl;

use crate::cpu::CPU;

fn main() {
    let mut cpu = CPU::new(String::from("roms/cpu_instrs.gb"), String::from("LR35902"));

    cpu.run();
}
