use crate::{common::*, state::State};

pub enum JPInstruction {
    JPPlain(Address),
    JPOffset(Register, Byte)
}

pub enum SEInstruction {
    SEPlain(Register, Byte),
    SNEPlain(Register, Byte),
    SERegister(Register, Register),
    SNERegister(Register, Register)
}

pub enum SKPInstruction {
    SKPPlain(Register),
    SKNP(Register)
}

pub enum LDInstruction {
    LDPlain(Register, Byte),
    LDRegister(Register, Register),
    LDI(DualWord),
    LDRDelay(Register),
    LDKey(Register),
    LDDelay(Register),
    LDSound(Register),
    LDSpritePos(Register),
    LDBCD(Register),
    LDRegDump(Register),
    LDRegLoad(Register)
}

pub enum ControlInstruction {
    CLS(),
    RET(),
    JP(JPInstruction),
    CALL(Address),
    SE(SEInstruction),
    SKP(SKPInstruction),
    DRW(Register, Register, Nibble),
    NOP()    
}

pub enum AddInstruction {
    ADDRegister(Register, Register),
    ADDPlain(Register, Byte),
    ADDI(Register),
}
pub enum MathInstruction {
    LD(LDInstruction),
    OR(Register, Register),
    AND(Register, Register),
    XOR(Register, Register),
    ADD(AddInstruction),
    SUB(Register, Register),
    SHR(Register, Register),
    SUBN(Register, Register),
    SHL(Register, Register),
    RND(Register, Byte)
}

pub enum Instruction {
    Control(ControlInstruction),
    Math(MathInstruction),
}

#[cfg(feature = "panic_on_unknown_inst")]
fn unknown_instruction(opcode: u16) -> ! {
    println!("Unknown instruction: {:04X}", opcode);
    #[cfg(debug_assertions)]
    panic!("Unknown instruction");
}

#[cfg(not(feature = "panic_on_unknown_inst"))]
fn unknown_instruction(opcode: u16) -> Instruction {
    eprintln!("Unknown instruction: {:04X}", opcode);
    Instruction::Control(ControlInstruction::NOP())
}

macro_rules! use_all_inst {
    () => {
        #[allow(unused_imports)]
        use Instruction::*;
        #[allow(unused_imports)]
        use ControlInstruction::*;
        #[allow(unused_imports)]
        use MathInstruction::*;
        #[allow(unused_imports)]
        use LDInstruction::*;
        #[allow(unused_imports)]
        use AddInstruction::*;
        #[allow(unused_imports)]
        use SEInstruction::*;
        #[allow(unused_imports)]
        use SKPInstruction::*;
        #[allow(unused_imports)]
        use JPInstruction::*;
    };
}

impl Instruction {
    fn decode(opcode: u16) -> Instruction {
        use_all_inst!();

        match opcode.get_byte(0) {
            0x0 => {
                match opcode.get_byte(1) {
                    0xE0 => Control(CLS()),
                    0xEE => Control(RET()),
                    _ => unknown_instruction(opcode)
                }
            },
            0x1 => Control(JP(JPPlain(opcode << 8))),
            0x2 => Control(CALL(opcode << 8)),
            0x3 => Control(SE(SEPlain(opcode.get_nibble(1), opcode.get_byte(2)))),
            0x4 => Control(SE(SNEPlain(opcode.get_nibble(1), opcode.get_byte(2)))),
            0x5 => Control(SE(SERegister(opcode.get_nibble(1), opcode.get_nibble(2)))),
            0x6 => Math(LD(LDPlain(opcode.get_nibble(1), opcode.get_byte(2)))),
            0x7 => Math(ADD(ADDPlain(opcode.get_nibble(1), opcode.get_byte(2)))),
            0x8 => {
                match opcode.get_nibble(3) {
                    0x0 => Math(LD(LDRegister(opcode.get_nibble(1), opcode.get_nibble(2)))),
                    0x1 => Math(OR(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0x2 => Math(AND(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0x3 => Math(XOR(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0x4 => Math(ADD(ADDRegister(opcode.get_nibble(1), opcode.get_nibble(2)))),
                    0x5 => Math(SUB(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0x6 => Math(SHR(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0x7 => Math(SUBN(opcode.get_nibble(1), opcode.get_nibble(2))),
                    0xE => Math(SHL(opcode.get_nibble(1), opcode.get_nibble(2))),
                    _ => unknown_instruction(opcode)
                }
            },
            0x9 => Control(SE(SNERegister(opcode.get_nibble(1), opcode.get_nibble(2)))),
            0xA => Math(LD(LDI(opcode << 8))),
            0xB => Control(JP(JPOffset(opcode.get_nibble(1), opcode.get_byte(2)))),
            0xC => Math(RND(opcode.get_nibble(1), opcode.get_byte(2))),
            0xD => Control(DRW(opcode.get_nibble(1), opcode.get_nibble(2), opcode.get_nibble(3))),
            0xE => {
                match opcode.get_byte(2) {
                    0x9E => Control(SKP(SKPPlain(opcode.get_nibble(1)))),
                    0xA1 => Control(SKP(SKNP(opcode.get_nibble(1)))),
                    _ => unknown_instruction(opcode)
                }
            },
            0xF => {
                match opcode.get_byte(2) {
                    0x07 => Math(LD(LDDelay(opcode.get_nibble(1)))),
                    0x0A => Math(LD(LDKey(opcode.get_nibble(1)))),
                    0x15 => Math(LD(LDDelay(opcode.get_nibble(1)))),
                    0x18 => Math(LD(LDSound(opcode.get_nibble(1)))),
                    0x1E => Math(ADD(ADDI(opcode.get_nibble(1)))),
                    0x29 => Math(LD(LDSpritePos(opcode.get_nibble(1)))),
                    0x33 => Math(LD(LDBCD(opcode.get_nibble(1)))),
                    0x55 => Math(LD(LDRegDump(opcode.get_nibble(1)))),
                    0x65 => Math(LD(LDRegLoad(opcode.get_nibble(1)))),
                    _ => unknown_instruction(opcode)
                }
            }
            _ => unknown_instruction(opcode)
        }
    }

    pub fn execute(&self, cpu: &mut State) {
        use_all_inst!();
        match self {
            Instruction::Control(instruction) => {
                match *instruction {
                    CLS() => cpu.framebuffer.lock().unwrap().clear(),
                    _ => unimplemented!()
                }
            },
            Instruction::Math(instruction) => {
                match *instruction {
                    _ => unimplemented!()
                }
            }
        }
    }
}