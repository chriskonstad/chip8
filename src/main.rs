extern crate libchip8;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use libchip8::Chip8;

fn main() {
    println!("Chip8 emulator in Rust");

    // TODO Setup the render system
    // setupGraphics();
    // setupInput();
    let path = Path::new("PONG");
    let display = path.display();

    // Initialize the emulator and load the game
    let mut chip = Chip8::new();
    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut game = Vec::new();
    match file.read_to_end(&mut game) {
        Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    };

    //let test = vec![0x62, 0x00, 0x61, 0x0B, 0xF1, 0x29, 0xD2, 0x05];
    //chip.loadHex(&test);
    chip.loadHex(&game);

    // Emulation loop
    loop {
        chip.emulateCycle();

        if chip.drawFlag {
            // TODO
            // drawGraphics();
            print!("{:?}", chip);
            chip.drawFlag = false;
        }

        // Store key press state
        //chip.setKeys();
    }
}
