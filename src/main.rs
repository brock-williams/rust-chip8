use chip8::Chip8;
use std::{env, fs::File, io::Read};

mod chip8;

fn main() {
    // Read ROM from args
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).unwrap()).unwrap();
    let mut file_data = Vec::<u8>::new();
    file.read_to_end(&mut file_data).expect("File not found.");

    // Setup Chip8
    let mut chip8: Chip8 = Chip8::new();
    chip8.load_rom(&file_data);
    chip8.execute_instruction();
    chip8.execute_instruction();
    chip8.execute_instruction();
    chip8.execute_instruction();
    chip8.execute_instruction();



    println!("Hello World!")
}