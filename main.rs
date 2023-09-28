use std::env;
use std::fs;
use std::io;

mod chip8;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    
    if arguments.len() != 2 {
        println!("Expected: './chip8 rom_name'");
        process::exit(1);
    }

    let mut rom_file = readFile(&arguments[1]);
    let mut rom_data = Vec::<u8>::new();

    rom_file
        .read_to_end(&mut rom_data)
        .expect("Failed to read file data.");

    // let mut chip8 = chip8::Chip8::new();
    // chip8.init_rom(&rom_data);
    // chip8.run();

}

fn readFile(filename: &String) -> fs::File {
    match fs::File::open(filename) {
        Ok(file) => return file,
        Err(e) => { 
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}
