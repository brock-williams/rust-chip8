// The original implementation of the Chip-8 language used a 64x32-pixel monochrome display with this format:

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    data: [[u8; WIDTH]; HEIGHT]
}

impl Display {
    pub fn new() -> Display {
        Display {
            data: [[0; WIDTH]; HEIGHT]
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value
    }
}
