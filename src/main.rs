extern crate libchip8;

use libchip8::Chip8;

fn main() {
    println!("Chip8 emulator in Rust");

    // TODO Setup the render system
    // setupGraphics();
    // setupInput();

    // Initialize the emulator and load the game
    let mut chip = Chip8::new();
    //chip.loadGame("pong");

    // Emulation loop
    loop {
        chip.emulateCycle();

        //if(chip.drawFlag) {
            // TODO
            // drawGraphics();
        //}

        // Store key press state
        //chip.setKeys();
    }
}
