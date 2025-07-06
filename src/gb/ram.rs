#[derive(Debug)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, val: u8) {
        self.memory[address as usize] = val;
    }

    pub fn write_word(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = (val & 0xFF) as u8;//lsb
        self.memory[address as usize + 1] = (val >> 8) as u8;//msb
    }
 }
