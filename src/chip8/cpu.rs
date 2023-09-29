use super::ram::Ram;
use super::display::Display;


#[derive(Debug)]
pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16, // Program counter
    sp: u8,  // Stack pointer
    stack: [u16; 16]
}

struct OpCode {
    h: u8,
    x: u8,
    y: u8,
    n: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16]
        }
    }

    pub fn execute_instruction(&self, ram: &mut Ram, display: &mut Display) {
        let lo = ram.read(self.pc) as u16;
        let hi = ram.read(self.pc + 1) as u16;
        
        // Combine hi and lo bytes to 16-bit instruction
        let instruction = (hi << 8) | lo;

        let opcode = OpCode {
            h: ((instruction & 0xF000) >> 12) as u8, // Top 4 bits
            x: ((instruction & 0x0F00) >> 8) as u8, // Next 4 bits
            y: ((instruction & 0x00F0) >> 4) as u8, // Next 4 bits
            n: (instruction & 0x000F) as u8         // Last 4 bits
        };

        let _addr = (instruction & 0x0FFF) as usize; // Also called "nnn"
        let _byte = (instruction & 0x00FF) as usize; // Also called "kk"


        match opcode {
            OpCode { h, x, y, n } => println!("h is {}, x is {}, y is {}, n is {}", h, x, y, n),
            _ => println!("goodbye!"),
        }

        println!("executing {:#X}", instruction);
    }


}


