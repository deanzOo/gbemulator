use crate::rom::Rom;

pub struct Bus {
    rom: Rom
}

/**
0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
4000	7FFF	16 KiB ROM Bank 01~NN	From cartridge, switchable bank via mapper (if any)
8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
C000	CFFF	4 KiB Work RAM (WRAM)
D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
FE00	FE9F	Object attribute memory (OAM)
FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
FF00	FF7F	I/O Registers
FF80	FFFE	High RAM (HRAM)
FFFF	FFFF	Interrupt Enable register (IE)
*/

impl Bus {
    pub(crate) fn new(rom: Rom) -> Self {
        Self { rom }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        if address < 0x8000 {
            self.rom.get_byte(address)
        }  else if address < 0xA000 {
            panic!("Not implemented");
        } else if address < 0xC000 {
            self.rom.get_byte(address)
        } else {
            panic!("Not implemented");
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        // self.rom.set_byte(address, value);
        println!("(NOT IMPLEMENTED) Write to address: {:04X}, value: {:02X}", address, value);
        println!()
    }
}