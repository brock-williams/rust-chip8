mod cpu;
mod ram;
mod display;

const CHP8_OFFSET: u16 = 0x200;

pub struct Chip8 {
    memory: ram::Ram,
    cpu: cpu::Cpu,
    display: display::Display
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: ram::Ram::new(),
            cpu: cpu::Cpu::new(),
            display: display::Display::new()
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn execute_instruction(&mut self) {
        self.cpu.execute_instruction(&mut self.memory, &mut self.display);
    }

}