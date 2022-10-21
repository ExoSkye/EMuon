use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Framebuffer {
    data: [u8; 64 * 32],
}

impl Framebuffer {
    pub fn new() -> Arc<Mutex<Framebuffer>> {
        Arc::new(Mutex::new(Framebuffer {
            data: [0; 64 * 32],
        }))
    }

    pub fn clear(&mut self) {
        self.data = [0; 64 * 32];
    }

    pub fn write(&mut self, x: u8, y: u8, value: bool) {
        let index = (y as usize * 64) + x as usize;
        self.data[index] = value as u8;
    }
}