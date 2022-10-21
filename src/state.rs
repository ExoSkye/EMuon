use std::sync::{Arc, Mutex};
use crate::{memory::Memory, framebuffer::Framebuffer, stack::Stack};

#[derive(Debug, Clone)]
pub struct State {
    pub memory: Arc<Mutex<Memory>>,
    pub framebuffer: Arc<Mutex<Framebuffer>>,
    pub registers: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Stack<u16>,
}

impl State {
    pub fn new() -> State {
        State {
            memory: Memory::new(),
            framebuffer: Framebuffer::new(),
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: Stack::new(),
        }
    }
}