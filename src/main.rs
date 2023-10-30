use std::collections::HashMap;

struct Rom {
    raw: Vec<u8>
}

impl Rom {
    fn new(filepath: String) -> Self {
        let raw = match std::fs::read(filepath) {
            Ok(rom) => rom,
            Err(e) => panic!("Error reading file: {}", e),
        };
        Self { raw }
    }

    fn get_byte(&self, address: u16) -> u8 {
        self.raw[address as usize]
    }
}

struct Bus {
    rom: Rom
}

impl Bus {
    fn new(rom: Rom) -> Self {
        Self { rom }
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.rom.get_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.rom.raw[address as usize] = value;
    }
}

enum ConditionType {
    None,
    Z,
    NZ,
    C,
    NC
}

impl Clone for ConditionType {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Z => Self::Z,
            Self::NZ => Self::NZ,
            Self::C => Self::C,
            Self::NC => Self::NC
        }
    }
}

enum Register {
    None,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
    A8,
    A16,
    D8,
    D16,
    A00
}

impl Clone for Register {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::A => Self::A,
            Self::B => Self::B,
            Self::C => Self::C,
            Self::D => Self::D,
            Self::E => Self::E,
            Self::H => Self::H,
            Self::L => Self::L,
            Self::AF => Self::AF,
            Self::BC => Self::BC,
            Self::DE => Self::DE,
            Self::HL => Self::HL,
            Self::SP => Self::SP,
            Self::PC => Self::PC,
            Self::A8 => Self::A8,
            Self::A16 => Self::A16,
            Self::D8 => Self::D8,
            Self::D16 => Self::D16,
            Self::A00 => Self::A00
        }
    }
}

// instruction executor function pointer type, args: cpu
type InstructionExecutor = fn(&mut CPU);

// todo add is_source_memory and use it all over
struct Instruction {
    mnemonic: String,
    opcode: u8,
    condition: ConditionType,
    executor: InstructionExecutor,
    target: Register,
    source: Register,
    is_target_memory: bool,
    is_source_memory: bool,
    length: u8
}

impl Instruction {
    fn new(mnemonic: String, opcode: u8, condition: ConditionType, executor: InstructionExecutor, target: Register, source: Register, is_target_memory: bool, is_source_memory: bool, length: u8) -> Self {
        Self { mnemonic, opcode, condition, executor, target, source, is_target_memory, is_source_memory, length }
    }

    fn execute(&self, cpu: &mut CPU) {
        (self.executor)(cpu);
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Self { mnemonic: self.mnemonic.clone(), opcode: self.opcode, condition: self.condition.clone(), executor: self.executor, target: self.target.clone(), source: self.source.clone(), is_target_memory: self.is_target_memory, is_source_memory: self.is_source_memory, length: self.length }
    }
}

struct CPU {
    // GameBoy CPU registers
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
    instruction_set: HashMap<u8, Instruction>,
    cb_instruction_set: HashMap<u8, Instruction>,
    curr_instruction: Instruction,
    ime: bool,
    running: bool
}

impl CPU {
    fn new(bus: Bus) -> Self {
        let instruction_array = vec![
            Instruction::new(String::from("NOP"), 0x00, ConditionType::None, CPU::nop, Register::None, Register::None, false, false, 1),
            Instruction::new(String::from("LD BC,d16"), 0x01, ConditionType::None, CPU::ld, Register::BC, Register::D16, false, false, 1),
            Instruction::new(String::from("LD (BC),A"), 0x02, ConditionType::None, CPU::ld, Register::BC, Register::A, true, false, 1),
            Instruction::new(String::from("INC BC"), 0x03, ConditionType::None, CPU::inc, Register::BC, Register::None, false, false, 1),
            Instruction::new(String::from("INC B"), 0x04, ConditionType::None, CPU::inc, Register::B, Register::None, false, false, 1),
            Instruction::new(String::from("DEC B"), 0x05, ConditionType::None, CPU::dec, Register::B, Register::None, false, false, 1),
            Instruction::new(String::from("LD B,d8"), 0x06, ConditionType::None, CPU::ld, Register::B, Register::D8, false, false, 2),
            Instruction::new(String::from("RLCA"), 0x07, ConditionType::None, CPU::rlca, Register::None, Register::None, false, false, 1),
            Instruction::new(String::from("LD (a16),SP"), 0x08, ConditionType::None, CPU::ld, Register::A16, Register::SP, true, false, 3),
            Instruction::new(String::from("ADD HL,BC"), 0x09, ConditionType::None, CPU::add, Register::HL, Register::BC, false, false, 1),
            Instruction::new(String::from("LD A,(BC)"), 0x0A, ConditionType::None, CPU::ld, Register::A, Register::BC, true, false, 1),
            Instruction::new(String::from("DEC BC"), 0x0B, ConditionType::None, CPU::dec, Register::BC, Register::None, false, false, 1),
            Instruction::new(String::from("INC C"), 0x0C, ConditionType::None, CPU::inc, Register::C, Register::None, false, false, 1),
            Instruction::new(String::from("DEC C"), 0x0D, ConditionType::None, CPU::dec, Register::C, Register::None, false, false, 1),
            Instruction::new(String::from("LD C,d8"), 0x0E, ConditionType::None, CPU::ld, Register::C, Register::D8, false, false, 2),
            Instruction::new(String::from("RRCA"), 0x0F, ConditionType::None, CPU::rrca, Register::None, Register::None, false, false, 1),

            Instruction::new(String::from("STOP"), 0x10, ConditionType::None, CPU::stop, Register::None, Register::None, false, false, 2),
            Instruction::new(String::from("LD DE,d16"), 0x11, ConditionType::None, CPU::ld, Register::DE, Register::D16, false, false, 3),
            Instruction::new(String::from("LD (DE),A"), 0x12, ConditionType::None, CPU::ld, Register::DE, Register::A, true, false, 1),
            Instruction::new(String::from("INC DE"), 0x13, ConditionType::None, CPU::inc, Register::DE, Register::None, false, false, 1),
            Instruction::new(String::from("INC D"), 0x14, ConditionType::None, CPU::inc, Register::D, Register::None, false, false, 1),
            Instruction::new(String::from("DEC D"), 0x15, ConditionType::None, CPU::dec, Register::D, Register::None, false, false, 1),
            Instruction::new(String::from("LD D,d8"), 0x16, ConditionType::None, CPU::ld, Register::D, Register::D8, false, false, 2),
            Instruction::new(String::from("RLA"), 0x17, ConditionType::None, CPU::rla, Register::None, Register::None, false, false, 1),
            Instruction::new(String::from("JR r8"), 0x18, ConditionType::None, CPU::jr, Register::None, Register::None, false, false, 2),
            Instruction::new(String::from("ADD HL,DE"), 0x19, ConditionType::None, CPU::add, Register::HL, Register::DE, false, false, 1),

            Instruction::new(String::from("JR NZ,r8"), 0x20, ConditionType::NZ, CPU::jr, Register::None, Register::None, false, false, 2),
            Instruction::new(String::from("LD HL,d16"), 0x21, ConditionType::None, CPU::ld, Register::HL, Register::D16, false, false, 3),
            Instruction::new(String::from("JR Z,r8"), 0x28, ConditionType::Z, CPU::jr, Register::None, Register::None, false, false, 2),

            Instruction::new(String::from("LD SP,d16"), 0x31, ConditionType::None, CPU::ld, Register::SP, Register::D16, false, false, 3),
            Instruction::new(String::from("LD A,d8"), 0x3E, ConditionType::None, CPU::ld, Register::A, Register::D8, false, false, 2),

            Instruction::new(String::from("LD B,A"), 0x47, ConditionType::None, CPU::ld, Register::B, Register::A, false, false, 1),

            Instruction::new(String::from("LD A,B"), 0x78, ConditionType::None, CPU::ld, Register::A, Register::B, false, false, 1),
            Instruction::new(String::from("LD A,H"), 0x7C, ConditionType::None, CPU::ld, Register::A, Register::H, false, false, 1),
            Instruction::new(String::from("LD A,L"), 0x7D, ConditionType::None, CPU::ld, Register::A, Register::L, false, false, 1),

            Instruction::new(String::from("XOR A"), 0xAF, ConditionType::None, CPU::xor, Register::None, Register::A, false, false, 1),

            Instruction::new(String::from("OR C"), 0xB1, ConditionType::None, CPU::or, Register::None, Register::C, false, false, 1),

            Instruction::new(String::from("JP a16"), 0xC3, ConditionType::None, CPU::jp,  Register::None, Register::None, false, false, 3),
            Instruction::new(String::from("PUSH BC"), 0xC5, ConditionType::None, CPU::push, Register::None, Register::BC, false, false, 1),
            Instruction::new(String::from("RET"), 0xC9, ConditionType::None, CPU::ret, Register::None, Register::None, false, false, 1),
            Instruction::new(String::from("CALL a16"), 0xCD, ConditionType::None, CPU::call,  Register::A16, Register::None, false, false, 3),

            Instruction::new(String::from("LDH (a8),A"), 0xE0, ConditionType::None, CPU::ld, Register::A8, Register::A, true, false, 2),
            Instruction::new(String::from("LD (a16),A"), 0xEA, ConditionType::None, CPU::ld, Register::A16, Register::A, true, false, 3),

            Instruction::new(String::from("LDH, A,(a8)"), 0xF0, ConditionType::None, CPU::ld, Register::A, Register::A8, true, true, 2),
            Instruction::new(String::from("DI"), 0xF3, ConditionType::None, CPU::di, Register::None, Register::None, false, false, 1),
            Instruction::new(String::from("PUSH AF"), 0xF5, ConditionType::None, CPU::push, Register::None, Register::AF, false, false, 1),
            Instruction::new(String::from("CP d8"), 0xFE, ConditionType::None, CPU::cp, Register::None, Register::PC, false, false, 2)
        ];
        let instruction_set: HashMap<u8, Instruction> = instruction_array
            .into_iter()
            .map(|inst| (inst.opcode, inst)) // Clone the 'opcode' field to avoid moving 'inst'
            .collect();

        let cb_instruction_array = vec![
            Instruction::new(String::from("RES 0,A"), 0x87, ConditionType::None, CPU::res, Register::A00, Register::A, false, false, 2)
        ];
        let cb_instruction_set: HashMap<u8, Instruction> = cb_instruction_array
            .into_iter()
            .map(|inst| (inst.opcode, inst)) // Clone the 'opcode' field to avoid moving 'inst'
            .collect();
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
            pc: 0x0100,
            cycle_count: 0,
            bus,
            instruction_set,
            cb_instruction_set,
            curr_instruction: Instruction::new(String::from("INVALID"), 0xD3, ConditionType::None, CPU::invalid,  Register::None, Register::None, false, false, 0),
            ime: true,
            running: true
        }
    }

    fn invalid(&mut self) {
        panic!("Invalid instruction");
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

    fn read_word_at_pc(&mut self) -> u16 {
        let result = self.read_word(self.pc);
        self.pc += 2;
        result
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.cycle_count += 4;
        self.bus.read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.cycle_count += 4;
        self.bus.write_byte(address, value);
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    fn fetch_next_instruction(&mut self) {
        let opcode = self.read_byte_at_pc();
        if opcode == 0xCB {
            let cb_opcode = self.read_byte_at_pc();
            let instruction = self.cb_instruction_set.get(&cb_opcode);
            if instruction.is_none() {
                panic!("Unknown CB instruction: {:02X}", cb_opcode);
            }
            self.curr_instruction = instruction.unwrap().clone();
            return;
        }
        let instruction = self.instruction_set.get(&opcode);
        if instruction.is_none() {
            panic!("Unknown instruction: {:02X}", opcode);
        }
        self.curr_instruction = instruction.unwrap().clone();
    }

    fn execute(&mut self) {
        let instruction = self.curr_instruction.clone();
        instruction.execute(self);
    }

    fn run(&mut self) {
        while self.running {
            self.fetch_next_instruction();
            self.print_debug();
            self.execute();
        }
    }

    fn set_pc(&mut self, value: u16) {
        self.cycle_count += 4;
        self.pc = value;
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

    fn get_af(&mut self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    fn get_bc(&mut self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
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
        print!("FLAGS: ");
        if (self.f & 0x80) == 0x80 {
            print!("Z");
        } else {
            print!("-");
        }
        if (self.f & 0x40) == 0x40 {
            print!("N");
        } else {
            print!("-");
        }
        if (self.f & 0x20) == 0x20 {
            print!("H");
        } else {
            print!("-");
        }
        if (self.f & 0x10) == 0x10 {
            print!("C");
        } else {
            print!("-");
        }
        println!();
        // print current opcode
        match self.curr_instruction.length {
            0 => { /* do nothing */ },
            1 => { println!("OPCODE: {:02X}", self.curr_instruction.opcode); },
            2 => { println!("OPCODE: {:02X} {:02X}", self.curr_instruction.opcode, self.bus.read_byte(self.pc)); },
            3 => { println!("OPCODE: {:02X} {:02X} {:02X}", self.curr_instruction.opcode, self.bus.read_byte(self.pc), self.bus.read_byte(self.pc + 1)); },
            _ => { panic!("Unknown instruction length"); }
        }
        println!("MNEMONIC: {}", self.curr_instruction.mnemonic);
        println!();
    }

    fn check_condition(&self, condition: ConditionType) -> bool {
        match condition {
            ConditionType::None => true,
            ConditionType::Z => (self.f & 0x80) == 0x80,
            ConditionType::NZ => (self.f & 0x80) == 0x00,
            ConditionType::C => (self.f & 0x10) == 0x10,
            ConditionType::NC => (self.f & 0x10) == 0x00,
        }
    }

    fn nop(&mut self) {
    // do nothing
    }

    fn jp(&mut self) {
        let operand = self.read_word_at_pc();
        self.set_pc(operand);
    }

    fn cp(&mut self) {
        let (result, overflowed) = (0, false);
        let operand: u8;
        match self.curr_instruction.source {
            Register::PC => {
                operand = self.read_byte_at_pc();
                (result, overflowed) = self.a.overflowing_sub(operand);
            },
            _ => { panic!("Unknown source register for CP instruction"); }
        }
        // Z 1 H C
        self.set_z_flag(result == 0);
        self.set_n_flag(true);
        self.set_h_flag((self.a & 0x0F) < (operand & 0x0F));
        self.set_c_flag(overflowed);
    }

    fn jr(&mut self) {
        let condition= self.curr_instruction.condition.clone();
        let operand = self.read_byte_at_pc();
        if self.check_condition(condition) {
            self.set_pc((self.pc as i16 + (operand as i8) as i16) as u16);
        } else {
            // do nothing
        }
    }

    fn xor(&mut self) {
        let operand: u8;
        let source = self.curr_instruction.source.clone();
        match source {
            Register::A => { operand = self.a; },
            _ => { panic!("Unknown source register for XOR instruction"); }
        }
        self.a ^= operand;
        self.set_z_flag(self.a == 0);
    }

    fn dec(&mut self) {
        let target = self.curr_instruction.target.clone();
        let result: u16;
        match target {
            Register::DE => {
                self.e = self.e.wrapping_sub(1);
                result = self.e as u16;
            },
            Register::BC => {
                let bc = ((self.b as u16) << 8) | (self.c as u16);
                result = bc.wrapping_sub(1);
                self.set_bc(result);
            },
            Register::C => {
                self.c = self.c.wrapping_sub(1);
                result = self.c as u16;
            },
            Register::D => {
                self.d = self.d.wrapping_sub(1);
                result = self.d as u16;
            },
            _ => { panic!("Unknown target register for DEC instruction"); }
        }
        // Z 1 H -
        self.set_z_flag(result == 0);
        self.set_n_flag(true);
        self.set_h_flag(result == 0x0F);
    }

    fn ld(&mut self) {
        let target = self.curr_instruction.target.clone();
        let source = self.curr_instruction.source.clone();
        let is_target_memory = self.curr_instruction.is_target_memory;
        let is_source_memory = self.curr_instruction.is_source_memory;
        match (target, source, is_target_memory, is_source_memory) {
            (Register::BC, Register::A, true, _) => {
                let address = ((self.b as u16) << 8) | (self.c as u16);
                self.write_byte(address, self.a);
            },
            (Register::BC, Register::D16, false, _) => {
                let operand = self.read_word_at_pc();
                self.b = (operand >> 8) as u8;
                self.c = operand as u8;
            },
            (Register::A, Register::D8, _, _) => {
                self.a = self.read_byte_at_pc();
            },
            (Register::A16, Register::A, _, _) => {
                let operand = self.read_word_at_pc();
                self.write_byte(operand, self.a);
            },
            (Register::A8, Register::A, _, _) => {
                let operand = self.read_byte_at_pc();
                self.write_byte(0xFF00 + (operand as u16), self.a);
            },
            (Register::A, Register::A8, _, true) => {
                let operand = self.read_byte_at_pc();
                self.a = self.read_byte(0xFF00 + (operand as u16));
            },
            (Register::B, Register::A, _, _) => {
                self.b = self.a;
            },
            (Register::SP, Register::D16, _, _) => {
                let operand = self.read_word_at_pc();
                self.sp = operand;
            },
            (Register::HL, Register::D16, _, _) => {
                let operand = self.read_word_at_pc();
                self.h = (operand >> 8) as u8;
                self.l = operand as u8;
            },
            (Register::A, Register::L, _, _) => {
                self.a = self.l;
            },
            (Register::A, Register::H, _, _) => {
                self.a = self.h;
            },
            (Register::A, Register::B, _, _) => {
                self.a = self.b;
            },
            (Register::B, Register::D8, _, _) => {
                self.b = self.read_byte_at_pc();
            },
            (Register::A16, Register::SP, true, _) => {
                let operand = self.read_word_at_pc();
                self.write_byte(operand, (self.sp & 0xFF) as u8);
                self.write_byte(operand + 1, ((self.sp >> 8) & 0xFF) as u8);
            },
            (Register::A, Register::BC, true, _) => {
                let address = ((self.b as u16) << 8) | (self.c as u16);
                self.a = self.read_byte(address);
            },
            (Register::C, Register::D8, _, _) => {
                self.c = self.read_byte_at_pc();
            },
            (Register::DE, Register::D16, false, _) => {
                let operand = self.read_word_at_pc();
                self.d = (operand >> 8) as u8;
                self.e = operand as u8;
            },
            (Register::DE, Register::A, true, _) => {
                let address = ((self.d as u16) << 8) | (self.e as u16);
                self.write_byte(address, self.a);
            },
            (Register::D, Register::D8, _, _) => {
                self.d = self.read_byte_at_pc();
            },
            _ => { panic!("Unknown target/source register combination for LD instruction"); }
        }
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn call(&mut self) {
        let condition = self.curr_instruction.condition.clone();
        let operand = self.read_word_at_pc();
        if self.check_condition(condition) {
            self.write_byte(self.sp - 1, ((self.pc + 3) >> 8) as u8);
            self.write_byte(self.sp - 2, (self.pc + 3) as u8);
            self.sp -= 2;
            self.set_pc(operand);
        } else {
            // do nothing
        }
    }

    fn res(&mut self) {
        let target = self.curr_instruction.target.clone();
        let source = self.curr_instruction.source.clone();
        match (target, source) {
            (Register::A00, Register::A) => {
                // reset bit 0 of A
                self.a &= 0xFE;
            },
            _ => { panic!("Unknown target/source register combination for RES instruction"); }
        }
    }

    fn inc(&mut self) {
        let target = self.curr_instruction.target.clone();
        let result: u16;
        match target {
            Register::B => {
                self.b = self.b.wrapping_add(1);
                result = self.b as u16;
            },
            Register::BC => {
                let bc = ((self.b as u16) << 8) | (self.c as u16);
                result = bc.wrapping_add(1);
                self.set_bc(result);
            },
            Register::C => {
                self.c = self.c.wrapping_add(1);
                result = self.c as u16;
            },
            Register::DE => {
                let de = self.get_de();
                result = de.wrapping_add(1);
                self.set_de(result);
            },
            Register::D => {
                self.d = self.d.wrapping_add(1);
                result = self.d as u16;
            },
            _ => { panic!("Unknown target register for INC instruction"); }
        }
        // Z 0 H -
        self.set_z_flag(result == 0);
        self.set_n_flag(false);
        self.set_h_flag(result == 0x0F);
    }

    fn ret(&mut self) {
        let condition = self.curr_instruction.condition.clone();
        if self.check_condition(condition) {
            let val = self.read_word(self.sp);
            self.sp += 2;
            self.set_pc(val);
        } else {
            // do nothing
        }
    }

    fn push(&mut self) {
        let source = self.curr_instruction.source.clone();
        let data: u16;
        match source {
            Register::AF => { data = self.get_af() },
            Register::BC => { data = self.get_bc() },
            _ => { panic!("Unknown source register for PUSH instruction"); }
        }
        self.sp -= 2;
        self.write_word(self.sp, data);
    }

    fn or(&mut self) {
        let operand: u8;
        let source = self.curr_instruction.source.clone();
        match source {
            Register::C => { operand = self.c; },
            _ => { panic!("Unknown source register for OR instruction"); }
        }
        self.a |= operand;
        self.set_z_flag(self.a == 0);
    }

    fn rlca(&mut self) {
        let carry = (self.a & 0x80) == 0x80;
        self.a <<= 1;
        if carry {
            self.a |= 0x01;
        }
        self.set_z_flag(self.a == 0);
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(carry);
    }

    fn add(&mut self) {
        let target = self.curr_instruction.target.clone();
        let source = self.curr_instruction.source.clone();
        let result: u16;
        let overflowed: bool;
        match (target, source) {
            (Register::HL, Register::BC) => {
                let hl = self.get_hl();
                let bc = self.get_bc();
                (result, overflowed) = hl.overflowing_add(bc);
                self.set_hl(result);
            },
            (Register::HL, Register::DE) => {
                let hl = self.get_hl();
                let de = self.get_de();
                (result, overflowed) = hl.overflowing_add(de);
                self.set_hl(result);
            },
            _ => { panic!("Unknown target/source register combination for ADD instruction"); }
        }
        // Z 0 H C
        self.set_z_flag(result == 0);
        self.set_n_flag(false);
        self.set_h_flag((result & 0x0FFF) < (self.h as u16));
        self.set_c_flag(overflowed);
    }

    fn rrca(&mut self) {
        let carry = (self.a & 0x01) == 0x01;
        self.a >>= 1;
        if carry {
            self.a |= 0x80;
        }
        self.set_z_flag(self.a == 0);
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(carry);
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn rla(&mut self) {
        let carry = (self.a & 0x80) == 0x80;
        self.a <<= 1;
        if (self.f & 0x10) == 0x10 {
            self.a |= 0x01;
        }
        self.set_z_flag(self.a == 0);
        self.set_n_flag(false);
        self.set_h_flag(false);
        self.set_c_flag(carry);
    }
}

fn main() {
    let rom = Rom::new(String::from("roms/cpu_instrs.gb"));
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);

    cpu.run();
}
