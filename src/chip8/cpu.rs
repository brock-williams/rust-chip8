use super::ram::Ram;

#[derive(Debug)]
pub struct Cpu {
    regs: [u8; 16],
    i: u16,
    pc: u16, // Program counter
    sp: u8,  // Stack pointer
    stack: [u16; 16]
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu {
            regs: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0, 16]
        }
    }

    pub fn execute_instruction(&self, ram: &Ram) {
        let lo = ram.read(self.pc) as u16;
        let hi = ram.read(self.pc + 1) as u16;
        
        // Combine hi and lo bytes to 16-bit instruction
        let opcode = (hi << 8) | lo;

        println!("executing {:#X}", opcode);
    }


}


