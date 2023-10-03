use super::ram::Ram;
use super::display::{Display, HEIGHT. WIDTH};
use super::ram::Ram;

const OPCODE_SIZE: u16 = 2;


#[derive(Debug)]
pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16, // Program counter
    sp: u8,  // Stack pointer
    stack: [u16; 16],
    dt: u16,
    st: u16
}

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

struct OpCode {
    h: u8,
    x: u8,
    y: u8,
    n: u8
}

impl OpCode {
    pub fn parse(instruction: u16) -> OpCode {
        OpCode {
            h: ((instruction & 0xF000) >> 12) as u8, // Top 4 bits
            x: ((instruction & 0x0F00) >> 8) as u8, // Next 4 bits
            y: ((instruction & 0x00F0) >> 4) as u8, // Next 4 bits
            n: (instruction & 0x000F) as u8         // Last 4 bits
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            dt: 0,
            st: 0
        }
    }

    pub fn execute_instruction(&self, ram: &mut Ram, _display: &mut Display) {
        let lo = ram.read(self.pc) as u16;
        let hi = ram.read(self.pc + 1) as u16;
        
        // Combine hi and lo bytes to 16-bit instruction
        let instruction = (hi << 8) | lo;

        let opcode = OpCode::parse(instruction);

        let _addr = (instruction & 0x0FFF) as u16; // Also called "nnn"
        let _byte = (instruction & 0x00FF) as u8; // Also called "kk"


        match opcode {
            OpCode { h, x, y, n } => println!("h is {}, x is {}, y is {}, n is {}", h, x, y, n),
            _ => println!("goodbye!"),
        }

        println!("executing {:#X}", instruction);
    }

    // 00E0 - CLS
    // Clear the display.
    fn op_cls(&self, display: &mut Display) -> ProgramCounter {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                display.write(x,y, 0);
            }

        }
        ProgramCounter::Next
    }

    // 00EE - RET
    // Return from a subroutine.

    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    fn op_ret(&self) -> ProgramCounter {
        self.sp -= 1;
        ProgramCounter::Jump(self.stack[self.sp as usize])
    }
    // 1nnn - JP addr
    // Jump to location nnn.

    // The interpreter sets the program counter to nnn.
    fn op_jmp_addr(&self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }
    
    // 2nnn - CALL addr
    // Call subroutine at nnn.
    
    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn op_call_addr(&self, nnn: u16) -> ProgramCounter {
        self.stack[self.sp as usize] = self.pc + OPCODE_SIZE;
        self.sp += 1;
        ProgramCounter::Jump(nnn)
    }

    // 3xkk - SE Vx, byte
    // Skip next instruction if Vx = kk.

    // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2
    fn op_se_byte(&self, vx: u8, kk: u8) -> ProgramCounter {
        let vx_reg = self.regs[vx as usize];

        if vx_reg == kk {
            ProgramCounter::Skip
        }
        else {
            ProgramCounter::Next
        }
    }

    // 4xkk - SNE Vx, byte
    // Skip next instruction if Vx != kk.

    // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    fn op_sne_byte(&self, vx: u8, kk: u8) -> ProgramCounter {
        let vx_reg = self.regs[vx as usize];

        if vx_reg != kk {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
    
    // 5xy0 - SE Vx, Vy
    // Skip next instruction if Vx = Vy.

    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn op_se_regs(&self, vx: u8, vy: u8) -> ProgramCounter {
        let vx_reg = self.regs[vx as usize];
        let vy_reg = self.regs[vy as usize];

        if vx_reg == vy_reg {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }


}


