
use std::collections::HashMap;
use crate::instruction_impl::{InstructionImpl, load_instructions};
use crate::bus::Bus;
use crate::rom::Rom;

pub struct CPU {
    // LR35902 CPU
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    cycle_count: u32,
    bus: Bus,
    instruction_set: HashMap<u8, InstructionImpl>,
    cb_instruction_set: HashMap<u8, InstructionImpl>,
    curr_opcode: u8,
    ime: bool,
    running: bool
}

impl CPU {
    pub(crate) fn new(filepath: String, cpu_type: String) -> Self {
        let rom = Rom::new(filepath);
        let bus = Bus::new(rom);
        let (instruction_impls, cb_instruction_impls) = load_instructions(cpu_type);

        let first_pc = 0x100;
        let mut first_opcode = bus.read_byte(first_pc);
        if first_opcode == 0xCB {
            first_opcode = bus.read_byte(first_pc + 1);
        }

        Self {
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0xFFFE,
            pc: first_pc,
            cycle_count: 0,
            bus,
            instruction_set: instruction_impls,
            cb_instruction_set: cb_instruction_impls,
            curr_opcode: first_opcode,
            ime: true,
            running: true
        }
    }

    fn read_byte_at_pc(&mut self) -> u8 {
        let result = self.read_byte(self.pc);
        self.pc += 1;
        result
    }

    fn read_word(&mut self, address: u16) -> u16 {
        let low = self.read_byte(address);
        let high = self.read_byte(address + 1);
        ((high as u16) << 8) | (low as u16)
    }

    pub(crate) fn read_word_at_pc(&mut self) -> u16 {
        let low = self.read_byte_at_pc();
        let high = self.read_byte_at_pc();
        ((high as u16) << 8) | (low as u16)
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.cycle_count += 4;
        self.bus.read_byte(address)
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        self.cycle_count += 4;
        self.bus.write_byte(address, value);
    }

    fn fetch(&mut self) {
        let mut opcode = self.read_byte_at_pc();
        if opcode == 0xCB {
            opcode = self.read_byte_at_pc();
        }
        self.curr_opcode = opcode;
    }

    fn execute(&mut self) {
        let instruction = self.instruction_set.get(&self.curr_opcode);
        if instruction.is_none() {
            println!("Unknown opcode: {:#04x}", self.curr_opcode);
            self.running = false;
            return;
        }
        let instruction = instruction.unwrap();
        (instruction.executor)(self);
    }

    pub(crate) fn run(&mut self) {
        while self.running {
            self.fetch();
            self.print_debug();
            self.execute();
        }
    }

    pub(crate) fn set_ime(&mut self, value: bool) {
        self.ime = value;
    }

    pub(crate) fn set_pc(&mut self, value: u16) {
        self.cycle_count += 4;
        self.pc = value;
    }

    fn flag_z(&self) -> bool {
        (self.f & 0x80) == 0x80
    }

    fn flag_n(&self) -> bool {
        (self.f & 0x40) == 0x40
    }

    fn flag_h(&self) -> bool {
        (self.f & 0x20) == 0x20
    }

    fn flag_c(&self) -> bool {
        (self.f & 0x10) == 0x10
    }

    fn set_z_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    fn set_h_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    fn set_c_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }

    fn set_n_flag(&mut self, value: bool) {
        if value {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    pub(crate) fn get_a(&mut self) -> u8 {
        self.a
    }

    fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    fn get_sp(&mut self) -> u16 {
        self.sp
    }

    pub(crate) fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    fn get_af(&mut self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub(crate) fn get_bc(&mut self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub(crate) fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    fn get_hl(&mut self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    fn get_de(&mut self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    fn print_debug(&self) {
        println!("A: {:02X} F: {:02X}", self.a, self.f);
        println!("B: {:02X} C: {:02X}", self.b, self.c);
        println!("D: {:02X} E: {:02X}", self.d, self.e);
        println!("H: {:02X} L: {:02X}", self.h, self.l);
        println!("SP: {:04X} PC: {:04X}", self.sp, self.pc - 1);
        println!("CYCLES: {}", self.cycle_count - 4);
        // print flags
        print!("FLAGS (znhc): ");
        if (self.f & 0x80) == 0x80 {
            print!("1");
        } else {
            print!("0");
        }
        if (self.f & 0x40) == 0x40 {
            print!("1");
        } else {
            print!("0");
        }
        if (self.f & 0x20) == 0x20 {
            print!("1");
        } else {
            print!("0");
        }
        if (self.f & 0x10) == 0x10 {
            print!("1");
        } else {
            print!("0");
        }
        println!();
        let opcode = self.bus.read_byte(self.pc - 1);
        let is_cb = self.bus.read_byte(self.pc - 2);
        let instruction_impl;
        if is_cb != 0xCB {
            instruction_impl = self.instruction_set.get(&opcode);
        } else {
            instruction_impl = self.cb_instruction_set.get(&opcode);
        }
        if instruction_impl.is_none() {
            return;
        }
        match instruction_impl.unwrap().instruction.length {
            1 => instruction_impl.unwrap().instruction.print_debug(0, 0),
            2 => instruction_impl.unwrap().instruction.print_debug(self.bus.read_byte(self.pc), 0),
            3 => instruction_impl.unwrap().instruction.print_debug(self.bus.read_byte(self.pc), self.bus.read_byte(self.pc + 1)),
            _ => panic!("Invalid instruction length")
        }
        println!();
    }
}