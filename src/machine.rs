use std::io::prelude::*;
use std::io::Result;
use std::io::BufReader;
use std::fs::File;
use std::fmt;
use std::fmt::Display;
use itertools::Itertools;
use rand::prelude::*;

use crate::isa::ISA;
use crate::isa::decode;

#[derive(Debug)]
pub struct CPU {
    pub r: [u8; 16], // general purpose registers
    pub i: usize,    // adressing register (16 bits)
    pub dt: u8,      // delay time
    pub st: u8,      // sound timer
    pub pc: usize,   // program counter (16 bits)
    pub sp: usize    // stack pointer (8 bits)
}

impl Default for CPU {
    fn default() -> Self {
        return CPU {
            r: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0
        }
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC:${:03X} SP:${:02X} I:${:03X} ", self.pc, self.sp, self.i)?;
        write!(f, "DT:${:02X} ST:${:02X} R:$", self.dt, self.st)?;
        for i in 0..self.r.len() {
            write!(f, "{:02X}", self.r[i])?;
        };
        fmt::Result::Ok(())
    }
}

pub struct Memory {
    pub rom: [u8; 80],      // up to 512 bytes
    pub ram: [u8; 4096],    // 4k RAM
    pub stack: [u16; 24],   // 24 call depth
    pub fb: [u8; 64*32],  // 64x32 pixels framebuffer
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            rom: [
                0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                0x20, 0x60, 0x20, 0x20, 0x70, // 1
                0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                0xF0, 0x80, 0xF0, 0x80, 0x80, // F
            ],
            ram: [0; 4096],
            stack: [0; 24],
            fb: [0; 64*32]
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.fb.chunks(64) {
            for pixel in line.iter() {
                if *pixel != 0 {
                    write!(f, " ")?;
                } else {
                    write!(f, "0")?;
                }
            }
            write!(f, "\n")?;
        };
        fmt::Result::Ok(())
    }
}

impl Memory {
    fn load(&mut self, filename: &str, addr: usize) -> Result<usize> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut pos = addr;
        // copy ROM into RAM
        self.ram[0..self.rom.len()].copy_from_slice(&self.rom[..]);
        // copy PROG into RAM
        while let Ok(ret) = reader.read(&mut self.ram[pos..]) {
            if ret > 0 {
                pos += ret;
            }
            else {
                break;
            }
        };
        Ok(pos - addr)
    }

    fn opcode(&mut self, addr: usize) -> &[u8] {
        &self.ram[addr .. (addr + 2)]
    }
}

pub struct Machine {
    pub keys: [bool; 16],
    cpu: CPU,
    memory: Memory,
}

impl Machine {

    pub fn new() -> Machine {
        Machine {
            cpu: CPU::default(),
            memory: Memory::default(),
            keys: [false; 16],
        }
    }

    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }

    pub fn mem(&self) -> &Memory {
        &self.memory
    }

    pub fn reset(&mut self) {
        self.cpu.pc = 0x200;  // common entry point
        self.cpu.sp = self.memory.stack.len() -1;  // end of stack
        self.memory.stack[self.cpu.sp] = self.cpu.pc as u16;  // entrypoint
    }

    pub fn load(&mut self, filename: &str) -> Result<usize> {
        self.reset();
        self.memory.load(filename, self.cpu.pc)
    }

    pub fn step(&mut self) -> Option<(usize, ISA)> {
        let pc = self.cpu.pc.clone();
        let op = decode(self.memory.opcode(self.cpu.pc))?;
        match op {
            ISA::CLS => {
                self.memory.fb.clone_from_slice(&[0; 64*32]);
                self.cpu.pc += 2;
            },
            ISA::RET => {
                self.cpu.pc = self.memory.stack[self.cpu.sp] as usize;
                self.cpu.sp += 1;
            },
            ISA::SYS(n) => {
                self.cpu.pc = n;
            },
            ISA::JP(n) => {
                self.cpu.pc = n;
            },
            ISA::CALL(n) => {
                self.cpu.sp -= 1;
                self.cpu.pc += 2;
                self.memory.stack[self.cpu.sp] = self.cpu.pc as u16;
                self.cpu.pc = n;
            },
            ISA::SKE(x, n) => {
                if self.cpu.r[x] == n {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::SKNE(x, n) => {
                if self.cpu.r[x] != n {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::SKRE(x, y) => {
                if self.cpu.r[x] == self.cpu.r[y] {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::LOAD(x, n) => {
                self.cpu.r[x] = n;
                self.cpu.pc += 2;
            },
            ISA::ADD(x, n) => {
                self.cpu.r[x] = self.cpu.r[x].wrapping_add(n);
                self.cpu.pc += 2;
            },
            ISA::MOVE(x, y) => {
                self.cpu.r[x] = self.cpu.r[y];
                self.cpu.pc += 2;
            },
            ISA::OR(x, y) => {
                self.cpu.r[x] |= self.cpu.r[y];
                self.cpu.pc += 2;
            },
            ISA::AND(x, y) => {
                self.cpu.r[x] &= self.cpu.r[y];
                self.cpu.pc += 2;
            },
            ISA::XOR(x, y) => {
                self.cpu.r[x] ^= self.cpu.r[y];
                self.cpu.pc += 2;
            },
            ISA::ADDR(x, y) => {
                self.cpu.r[0xf] = match self.cpu.r[x].checked_add(self.cpu.r[y]) {
                    Some(val) => {self.cpu.r[x] = val; 0}
                    None => {self.cpu.r[x] = self.cpu.r[x].wrapping_add(self.cpu.r[y]); 1}
                };
                self.cpu.pc += 2;
            },
            ISA::SUB(x, y) => {
                self.cpu.r[0xf] = match self.cpu.r[x].checked_sub(self.cpu.r[y]) {
                    Some(val) => {self.cpu.r[x] = val; 1}
                    None => {self.cpu.r[x] = self.cpu.r[x].wrapping_sub(self.cpu.r[y]); 0}
                };
                self.cpu.pc += 2;
            },
            ISA::SHR(x, _y) => {
                self.cpu.r[0xf] = self.cpu.r[x] & 0x1;
                self.cpu.r[x] = self.cpu.r[x].wrapping_div(2);
                self.cpu.pc += 2;
            },
            ISA::SHL(x, _y) => {
                self.cpu.r[0xf] = (self.cpu.r[x] & 0x80) >> 7;
                self.cpu.r[x] = self.cpu.r[x].wrapping_mul(2);
                self.cpu.pc += 2;
            },
            ISA::SUBN(x, y) => {
                self.cpu.r[0xf] = match self.cpu.r[y].checked_sub(self.cpu.r[x]) {
                    Some(val) => {self.cpu.r[x] = val; 1}
                    None => {self.cpu.r[x] = self.cpu.r[y].wrapping_sub(self.cpu.r[x]); 0}
                };
                self.cpu.pc += 2;
            },
            ISA::SKRNE(x, y) => {
                if self.cpu.r[x] != self.cpu.r[y] {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::LOADI(n) => {
                self.cpu.i = n;
                self.cpu.pc += 2;
            },
            ISA::JUMPI(n) => {
                self.cpu.pc = self.cpu.r[0] as usize + n;
            },
            ISA::RAND(x, n) => {
                self.cpu.r[x] = rand::thread_rng().gen_range(0..255) & n;
                self.cpu.pc += 2;
            },
            ISA::DRAW(x, y, n) => {
                self.cpu.r[0xf] = 0;
                let px = self.cpu.r[x] as usize;
                let py = self.cpu.r[y] as usize;
                for (i, j) in (0..8).cartesian_product(0..n) {
                    let pixel = 64 * ((py + j) % 32) + (px + i) % 64;
                    if (self.memory.ram[self.cpu.i + j] & (0x80 >> i)) != 0 {
                        if self.memory.fb[pixel] != 0 {
                            self.cpu.r[0xf] = 1;
                        }
                        self.memory.fb[pixel] = !self.memory.fb[pixel];
                    }
                }
                self.cpu.pc += 2;
            },
            ISA::SKPR(x) => {
                if self.keys[self.cpu.r[x] as usize] == true {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::SKUP(x) => {
                if self.keys[self.cpu.r[x] as usize] == false {
                    self.cpu.pc += 4;
                } else {
                    self.cpu.pc += 2;
                }
            },
            ISA::MOVED(x) => {
                self.cpu.r[x] = self.cpu.dt;
                self.cpu.pc += 2;
            },
            ISA::KEYD(x) => {
                if let Some(key) = self.keys.iter().position(|&e| e) {
                    self.keys[x] = false;
                    self.cpu.r[x] = key as u8;
                    self.cpu.pc += 2;
                }
            },
            ISA::LOADD(x) => {
                self.cpu.dt = self.cpu.r[x];
                self.cpu.pc += 2;
            },
            ISA::LOADS(x) => {
                self.cpu.st = self.cpu.r[x];
                self.cpu.pc += 2;
            },
            ISA::ADDI(x) => {
                self.cpu.i += self.cpu.r[x] as usize;
                self.cpu.pc += 2;
            },
            ISA::LDSPR(x) => {
                self.cpu.i = ((self.cpu.r[x] & 0xf) * 5) as usize;
                self.cpu.pc += 2;
            },
            ISA::BCD(x) => {
                let value = self.cpu.r[x];
                self.memory.ram[self.cpu.i] = (value / 100) % 10;
                self.memory.ram[self.cpu.i + 1] = (value / 10) % 10;
                self.memory.ram[self.cpu.i + 2] = value % 10;
                self.cpu.pc += 2;
            },
            ISA::STOR(n) => {
                for i in 0..(1+n) {
                    self.memory.ram[self.cpu.i + i] = self.cpu.r[i];
                }
                self.cpu.pc += 2;
            },
            ISA::READ(n) => {
                for i in 0..(1+n) {
                    self.cpu.r[i] = self.memory.ram[self.cpu.i + i];
                }
                self.cpu.pc += 2;
            },
            ISA::NOP(opcode) => {
                panic!("Unexpected OP(${:03X}) at ${:03X} address", opcode, self.cpu.pc);
            }
        };
        Some((pc, op))
    }

    pub fn tick(&mut self) {
        // simluate timers
        if self.cpu.dt > 0 {
            self.cpu.dt -= 1;
        }
        if self.cpu.st > 0 {
            self.cpu.st -= 1;
        }
    }

}
