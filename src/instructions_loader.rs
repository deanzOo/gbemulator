use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Instruction {
    opcode: u8,
    mnemonic: String,
    pub(crate) length: u8,
    cycles: String,
    flags: Flags
}

enum FlagsAction {
    Set,
    Reset,
    Ignore
}

#[derive(Deserialize)]
struct Flags {
    z: String,
    n: String,
    h: String,
    c: String
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let z = match self.z.as_str() {
            "z" => "z",
            "0" => "Reset",
            "1" => "Set",
            "-" => "Ignore",
            _ => panic!("Invalid flag value")
        };
        let n = match self.n.as_str() {
            "n" => "n",
            "0" => "Reset",
            "1" => "Set",
            "-" => "Ignore",
            _ => panic!("Invalid flag value")
        };
        let h = match self.h.as_str() {
            "h" => "h",
            "0" => "Reset",
            "1" => "Set",
            "-" => "Ignore",
            _ => panic!("Invalid flag value")
        };
        let c = match self.c.as_str() {
            "c" => "c",
            "0" => "Reset",
            "1" => "Set",
            "-" => "Ignore",
            _ => panic!("Invalid flag value")
        };
        write!(f, "z: {}, n: {}, h: {}, c: {}", z, n, h, c)
    }
}

impl Instruction {
    fn new(opcode: u8, mnemonic: String, length: u8, cycles: String, flags: Flags) -> Instruction {
        Instruction {
            opcode,
            mnemonic,
            length,
            cycles,
            flags
        }
    }

    pub(crate) fn print_debug(&self, op1: u8, op2: u8) {
        match self.length {
            1 => println!("Instruction: {:#04x}, mnemonic: {}, length: {}, cycles: {}, flags: {:?}", self.opcode, self.mnemonic, self.length, self.cycles, self.flags),
            2 => println!("Instruction: {:#04x} {:#04x}, mnemonic: {}, length: {}, cycles: {}, flags: {:?}", self.opcode, op1, self.mnemonic, self.length, self.cycles, self.flags),
            3 => println!("Instruction: {:#04x} {:#04x} {:#04x}, mnemonic: {}, length: {}, cycles: {}, flags: {:?}", self.opcode, op1, op2, self.mnemonic, self.length, self.cycles, self.flags),
            _ => panic!("Invalid instruction length")
        }
    }
}

pub fn instructions_from_json(filepath: String) -> HashMap<u8, Instruction> {
    let mut file = File::open(filepath).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read file");

    let instructions: Result<Vec<Instruction>, serde_json::Error> = serde_json::from_str(&json_data);

    match instructions {
        Ok(instructions) => {
            let mut instructions_map = HashMap::new();
            for instruction in instructions {
                instructions_map.insert(instruction.opcode, instruction);
            }
            instructions_map
        },
        Err(e) => panic!("Error parsing JSON: {}", e)
    }
}