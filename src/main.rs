#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod common;
pub mod ast;
pub mod state;
pub mod memory;
pub mod framebuffer;
pub mod stack;

use state::State;

fn main() {
    let mut state = State::new();
    
}
