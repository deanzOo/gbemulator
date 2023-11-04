mod rom;
mod bus;
mod cpu;

mod instructions_loader;
mod instruction_impl;

use crate::cpu::CPU;
use crate::rom::Rom;
use crate::bus::Bus;


fn main() {
    let rom = Rom::new(String::from("roms/cpu_instrs.gb"));
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);

    cpu.run();
}
