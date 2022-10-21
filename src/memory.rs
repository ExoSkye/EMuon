use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Arc<Mutex<Memory>> {
        Arc::new(Mutex::new(Memory {
            data: [0; 4096],
        }))
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.data[address as usize] = (value >> 8) as u8;
        self.data[address as usize + 1] = (value & 0xFF) as u8;
    }

    pub fn write_dword(&mut self, address: u16, value: u32) {
        self.data[address as usize] = (value >> 24) as u8;
        self.data[address as usize + 1] = ((value >> 16) & 0xFF) as u8;
        self.data[address as usize + 2] = ((value >> 8) & 0xFF) as u8;
        self.data[address as usize + 3] = (value & 0xFF) as u8;
    }

    pub fn write_data(&mut self, address: u16, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.data[address as usize + i] = *byte;
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        ((self.data[address as usize] as u16) << 8) | (self.data[address as usize + 1] as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_rw() {
        let mut memory = Memory::new();
        memory.lock().unwrap().write(0x200, 0x12);
        assert_eq!(memory.lock().unwrap().read(0x200), 0x12);
    }

    #[test]
    fn test_memory_word_rw() {
        let mut memory = Memory::new();
        memory.lock().unwrap().write_word(0x200, 0x1234);
        assert_eq!(memory.lock().unwrap().read_word(0x200), 0x1234);
    }

    #[test]
    fn test_memory_write_data() {
        let mut memory = Memory::new();
        memory.lock().unwrap().write_data(0x200, &[0x12, 0x34, 0x56, 0x78]);
        assert_eq!(memory.lock().unwrap().read(0x200), 0x12);
        assert_eq!(memory.lock().unwrap().read(0x201), 0x34);
        assert_eq!(memory.lock().unwrap().read(0x202), 0x56);
        assert_eq!(memory.lock().unwrap().read(0x203), 0x78);
    }

    #[test]
    fn test_memory_dword_rw() {
        let mut memory = Memory::new();
        memory.lock().unwrap().write_dword(0x200, 0x12345678);
        assert_eq!(memory.lock().unwrap().read(0x200), 0x12);
        assert_eq!(memory.lock().unwrap().read(0x201), 0x34);
        assert_eq!(memory.lock().unwrap().read(0x202), 0x56);
        assert_eq!(memory.lock().unwrap().read(0x203), 0x78);
    }
}