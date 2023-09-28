
// The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095). 
// The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by programs

// Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536). Programs beginning at 0x600 are intended for the ETI 660 computer.
const RAM_SIZE: usize = 0xFFF;
const RAM_START: usize = 0x200;

#[derive(Debug)]
pub struct Ram {
    data: [u8; RAM_SIZE]
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram {
            data: [0, RAM_SIZE]
        };
        // Programs may also refer to a group of sprites representing the hexadecimal digits 0 through F. 
        // These sprites are 5 bytes long, or 8x5 pixels. The data should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF). 
        // Below is a listing of each character's bytes, in binary and hexadecimal:
        let digits = [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
        ];

        let mut i = 0;
        for digit in digits {
            for byte in digit {
                ram.data[i] = byte;
                i += 1
            }
        }
        
        ram
    }

    pub fn read(&mut self, i: i16 ) -> u8 {
        self.data[i as usize]
    }

    pub fn write(&mut self, i: u16, value: u8) {
        if i <= RAM_START as u16 { // First 512 bits protected
            panic!("First 512 bits are protected, tried writing to address {}", i);
        }

        self.data[i as usize] = value;
    }

    pub fn read_rom(&mut self, rom: &Vec<u8>) {
        if rom.len() > RAM_SIZE - RAM_START {
            panic!("Invalid rom. Length must be below {}", RAM_SIZE-RAM_START);
        }

        for idx in 0..rom.len() {
            // Must add RAM_START as offset for protected memory
            self.data[idx + RAM_START] = rom[idx];
        }
    }
}