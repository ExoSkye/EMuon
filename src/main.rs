mod memory;
mod decoder;
mod framebuffer;

fn main() {
    let mut memory = memory::Memory::new();
    memory.lock().unwrap().write(0x200, 0x12);
}
