pub struct Rom {
    raw: Vec<u8>
}

impl Rom {
    pub(crate) fn new(filepath: String) -> Self {
        let raw = match std::fs::read(filepath) {
            Ok(rom) => rom,
            Err(e) => panic!("Error reading file: {}", e),
        };
        Self { raw }
    }

    pub(crate) fn get_byte(&self, address: u16) -> u8 {
        self.raw[address as usize]
    }

    pub(crate) fn set_byte(&mut self, address: u16, value: u8) {
        self.raw[address as usize] = value;
    }
}