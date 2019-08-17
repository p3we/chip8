use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::convert::Into;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ISA {
    CLS,                       // (00E0) Clear display
    RET,                       // (00EE) Return from subroutine
    SYS(usize),                // (0nnn) Jump to machine code routine at nnn addr
    JP(usize),                 // (1nnn) Jump to nnn address
    CALL(usize),               // (2nnn) Call subroutine at nnn addr
    SKE(usize, u8),            // (3xnn) Skip next instruction if Vx == nn
    SKNE(usize, u8),           // (4xnn) Skip next instruction if Vx != nn
    SKRE(usize, usize),        // (5xy0) Skip next instruction if Vx == Vy
    LOAD(usize, u8),           // (6xnn) Load Vx with nn value
    ADD(usize, u8),            // (7xnn) Add value nn to Vx
    MOVE(usize, usize),        // (8xy0) Move value of Vx to Vy
    OR(usize, usize),          // (8xy1) Perform Vy = Vx | Vy
    AND(usize, usize),         // (8xy2) Perform Vy = Vx & Vy
    XOR(usize, usize),         // (8xy3) Perform Vy = Vx ^ Vy
    ADDR(usize, usize),        // (8xy4) Perform Vx = Vx + Vy
    SUB(usize, usize),         // (8xy5) Perform Vx = Vx - Vy
    SHR(usize, usize),         // (8xy6) Perform Vy = Vx >> 1
    SUBN(usize, usize),        // (8xy7) Perform Vy = Vy - Vx
    SHL(usize, usize),         // (8xyE) Perform Vy = Vx << 1
    SKRNE(usize, usize),       // (9xy0) Skip next instruction if Vx != Vy
    LOADI(usize),              // (Annn) Load index with value nnn
    JUMPI(usize),              // (Bnnn) Jump to nnn + index
    RAND(usize, u8),           // (Cxnn) Generate random number and store it in Vx
    DRAW(usize, usize, usize), // (Dxyn) Draw n sprint at (Vx, Vy) location
    SKPR(usize),               // (Ex9E) Skip next instruction if Vx key is pressed
    SKUP(usize),               // (ExA1) Skip next instruction if Vx key is not pressed
    MOVED(usize),              // (Fx07) Move DT value into Vx
    KEYD(usize),               // (Fx0A) Wait for key pressed and store value in Vx
    LOADD(usize),              // (Fx15) Load DT with Vx value
    LOADS(usize),              // (Fx18) Load ST with Vx value
    ADDI(usize),               // (Fx1E) Add value in Vx to index
    LDSPR(usize),              // (Fx29) Load index with address of sprite representing Vx value
    BCD(usize),                // (Fx33) Store BCD representation of Vx value at mem[index..index+2]
    STOR(usize),               // (Fx55) Store V0 to Vx register values at mem[index:index+x]
    READ(usize),               // (Fx65) Load V0 to Vx register values with mem[index:index+x]
    NOP(u16),                  // (????) Invalid operation
}

impl Display for ISA {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ISA::CLS => write!(f, "CLS"),
            ISA::RET => write!(f, "RET"),
            ISA::SYS(a) => write!(f, "SYS ${:03X}", a),
            ISA::JP(a) => write!(f, "JP ${:03X}", a),
            ISA::CALL(a) => write!(f, "CALL ${:03X}", a),
            ISA::SKE(x, n) => write!(f, "SKE r{:X}, ${:03X}", x, n),
            ISA::SKNE(x, n) => write!(f, "SKNE r{:X}, ${:03X}", x, n),
            ISA::SKRE(x, y) => write!(f, "SKRE r{:X}, r{:X}", x, y),
            ISA::LOAD(x, n) => write!(f, "LOAD r{:X}, ${:03X}", x, n),
            ISA::ADD(x, n) => write!(f, "ADD r{:X}, ${:03X}", x, n),
            ISA::MOVE(x, y) => write!(f, "MOVE r{:X}, r{:X}", x, y),
            ISA::OR(x, y) => write!(f, "OR r{:X}, r{:X}", x, y),
            ISA::AND(x, y) => write!(f, "ADD r{:X}, r{:X}", x, y),
            ISA::XOR(x, y) => write!(f, "XOR r{:X}, r{:X}", x, y),
            ISA::ADDR(x, y) => write!(f, "ADDR r{:X}, r{:X}", x, y),
            ISA::SUB(x, y) => write!(f, "SUB r{:X}, r{:X}", x, y),
            ISA::SHR(x, y) => write!(f, "SHR r{:X}, r{:X}", x, y),
            ISA::SUBN(x, y) => write!(f, "SUBN r{:X}, r{:X}", x, y),
            ISA::SHL(x, y) => write!(f, "SHL r{:X}, r{:X}", x, y),
            ISA::SKRNE(x, y) => write!(f, "SKRNE r{:X}, r{:X}", x, y),
            ISA::LOADI(n) => write!(f, "LOADI ${:03X}", n),
            ISA::JUMPI(n) => write!(f, "JUMPI ${:03X}", n),
            ISA::RAND(x, n) => write!(f, "RAND r{:X}, ${:03X}", x, n),
            ISA::DRAW(x, y, n) => write!(f, "DRAW r{:X}, r{:X}, ${:X}", x, y, n),
            ISA::SKPR(x) => write!(f, "SKPR r{:X}", x),
            ISA::SKUP(x) => write!(f, "SKUP r{:X}", x),
            ISA::MOVED(x) => write!(f, "MOVED r{:X}", x),
            ISA::KEYD(x) => write!(f, "KEYD ${:03X}", x),
            ISA::LOADD(x) => write!(f, "LOADD r{:X}", x),
            ISA::LOADS(x) => write!(f, "LOADS r{:X}", x),
            ISA::ADDI(x) => write!(f, "ADDI r{:X}", x),
            ISA::LDSPR(x) => write!(f, "LDSPR r{:X}", x),
            ISA::BCD(x) => write!(f, "BCD r{:X}", x),
            ISA::STOR(x) => write!(f, "STOR ${:X}", x),
            ISA::READ(x) => write!(f, "READ ${:X}", x),
            ISA::NOP(c) => write!(f, "NOP ${:04X}", c),
        }
    }
}

pub fn decode<'a>(bytes: &[u8]) -> Option<ISA> {
    let high = bytes.get(0)?;
    let low = bytes.get(1)?;
    let opcode = ((*high as u16) << 8) | (*low as u16);
    let op = match opcode & 0xf000 {
        0x0000 => {
            if opcode == 0x00e0 {
                ISA::CLS
            }
            else if opcode == 0x00ee {
                ISA::RET
            }
            else {
                ISA::SYS((opcode & 0x0fff).into())
            }
        }
        0x1000 => ISA::JP((opcode & 0x0fff).into()),
        0x2000 => ISA::CALL((opcode & 0x0fff).into()),
        0x3000 => ISA::SKE(((opcode & 0x0f00) >> 8).into(), (opcode & 0x00ff) as u8),
        0x4000 => ISA::SKNE(((opcode & 0x0f00) >> 8).into(), (opcode & 0x00ff) as u8),
        0x5000 => ISA::SKRE(((opcode & 0x0f00) >> 8).into(), ((opcode & 0x00f0) >> 4).into()),
        0x6000 => ISA::LOAD(((opcode & 0x0f00) >> 8).into(), (opcode & 0x00ff) as u8),
        0x7000 => ISA::ADD(((opcode & 0x0f00) >> 8).into(), (opcode & 0x00ff) as u8),
        0x8000 => {
            let x: usize = ((opcode & 0x0f00) >> 8).into();
            let y: usize = ((opcode & 0x00f0) >> 4).into();
            match opcode & 0xf {
                0x0 => ISA::MOVE(x, y),
                0x1 => ISA::OR(x, y),
                0x2 => ISA::AND(x, y),
                0x3 => ISA::XOR(x, y),
                0x4 => ISA::ADDR(x, y),
                0x5 => ISA::SUB(x, y),
                0x6 => ISA::SHR(x, y),
                0x7 => ISA::SUBN(x, y),
                0xE => ISA::SHL(x, y),
                _ => ISA::NOP(opcode)
            }
        }
        0x9000 => ISA::SKRNE(((opcode & 0x0f00) >> 8).into(), ((opcode & 0x00f0) >> 4).into()),
        0xA000 => ISA::LOADI((opcode & 0x0fff).into()),
        0xB000 => ISA::JUMPI((opcode & 0x0fff).into()),
        0xC000 => ISA::RAND(((opcode & 0x0f00) >> 8).into(), (opcode & 0x00ff) as u8),
        0xD000 => ISA::DRAW(
            ((opcode & 0x0f00) >> 8).into(),
            ((opcode & 0x00f0) >> 4).into(),
            ((opcode & 0x000f) >> 0).into()
        ),
        0xE000 => {
            let x: usize = ((opcode & 0x0f00) >> 8).into();
            match opcode & 0xff {
                0x9E => ISA::SKPR(x),
                0xA1 => ISA::SKUP(x),
                _ => ISA::NOP(opcode)
            }
        }
        0xF000 => {
            let x: usize = ((opcode & 0x0f00) >> 8).into();
            match opcode & 0xff {
                0x07 => ISA::MOVED(x),
                0x0A => ISA::KEYD(x),
                0x15 => ISA::LOADD(x),
                0x18 => ISA::LOADS(x),
                0x1E => ISA::ADDI(x),
                0x29 => ISA::LDSPR(x),
                0x33 => ISA::BCD(x),
                0x55 => ISA::STOR(x),
                0x65 => ISA::READ(x),
                _ => ISA::NOP(opcode)
            }
        }
        _ => ISA::NOP(opcode)
    };
    Some(op)
}

#[cfg(test)]
#[test]
fn test_decode() {
    let tests: [([u8; 2], ISA); 35] = [
        ([0x00, 0xE0], ISA::CLS),
        ([0x00, 0xEE], ISA::RET),
        ([0x0F, 0xFF], ISA::SYS(4095)),
        ([0x1F, 0xFF], ISA::JP(4095)),
        ([0x2F, 0xFF], ISA::CALL(4095)),
        ([0x3F, 0xFF], ISA::SKE(15, 255)),
        ([0x4F, 0xFF], ISA::SKNE(15, 255)),
        ([0x5F, 0xFF], ISA::SKRE(15, 15)),
        ([0x6F, 0xFF], ISA::LOAD(15, 255)),
        ([0x7F, 0xFF], ISA::ADD(15, 255)),
        ([0x8F, 0xF0], ISA::MOVE(15, 15)),
        ([0x8F, 0xF1], ISA::OR(15, 15)),
        ([0x8F, 0xF2], ISA::AND(15, 15)),
        ([0x8F, 0xF3], ISA::XOR(15, 15)),
        ([0x8F, 0xF4], ISA::ADDR(15, 15)),
        ([0x8F, 0xF5], ISA::SUB(15, 15)),
        ([0x8F, 0xF6], ISA::SHR(15, 15)),
        ([0x8F, 0xF7], ISA::SUBN(15, 15)),
        ([0x8F, 0xFE], ISA::SHL(15, 15)),
        ([0x9F, 0xFF], ISA::SKRNE(15, 15)),
        ([0xAF, 0xFF], ISA::LOADI(4095)),
        ([0xBF, 0xFF], ISA::JUMPI(4095)),
        ([0xCF, 0xFF], ISA::RAND(15, 255)),
        ([0xDF, 0xFF], ISA::DRAW(15, 15, 15)),
        ([0xEF, 0x9E], ISA::SKPR(15)),
        ([0xEF, 0xA1], ISA::SKUP(15)),
        ([0xFF, 0x07], ISA::MOVED(15)),
        ([0xFF, 0x0A], ISA::KEYD(15)),
        ([0xFF, 0x15], ISA::LOADD(15)),
        ([0xFF, 0x18], ISA::LOADS(15)),
        ([0xFF, 0x1E], ISA::ADDI(15)),
        ([0xFF, 0x29], ISA::LDSPR(15)),
        ([0xFF, 0x33], ISA::BCD(15)),
        ([0xFF, 0x55], ISA::STOR(15)),
        ([0xFF, 0x65], ISA::READ(15)),
    ];
    for (data, expected) in tests.iter() {
        let op = decode(&data[..]);
        assert!(op.is_some());
        assert_eq!(op.unwrap(), *expected);
    }
}
