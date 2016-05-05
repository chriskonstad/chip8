extern crate chip8;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

mod audio;
mod graphics;
mod input;

use chip8::Chip8;
use clap::{Arg, App};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

const SCALE : u32 = 8;
const WIDTH : u32 = 64 * SCALE;
const HEIGHT : u32 = 32 * SCALE;

fn version() -> &'static str {
    concat!(env!("CARGO_PKG_VERSION_MAJOR"),
    ".",
    env!("CARGO_PKG_VERSION_MINOR"),
    ".",
    env!("CARGO_PKG_VERSION_PATCH")
    )
}

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("Chip8 Emulator")
        .version(version())
        .author("Chris Konstad <chriskon149@gmail.com>")
        .about("Runs Chip8 games.")
        .arg(Arg::with_name("ROM")
             .help("Sets the path to the ROM to play")
             .required(true))
        .get_matches();

    println!("Chip8 emulator starting...");

    // Initialize the emulator and load the game
    let path = Path::new(matches.value_of("ROM").unwrap());
    let display = path.display();

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
    chip.load_hex(&game);

    // Prepare SDL for video, audio, and input
    let sdl_context = sdl2::init().unwrap();

    // TODO TEST AT 60frames a second!
    // This isn't set to 60Hz because that was too slow
    let mut window = graphics::Display::new(&sdl_context,
                                            "Chip8 Emulator",
                                            WIDTH,
                                            HEIGHT,
                                            Duration::from_millis(2));
    let mut beeper = audio::Beeper::new(&sdl_context,
                                        Duration::from_millis(250));
    let mut keyboard = input::Keyboard::new(&sdl_context);

    // Emulation loop
    'running: loop {
        // Check the input and store it on the chip
        match keyboard.check(&mut chip.key) {
            input::Command::Quit => break 'running,
            input::Command::Continue => {}
        }

        // Run a cycle on the chip
        chip.emulate_cycle();

        // Render the frame if needed
        if chip.draw_flag {
            debug!("{:?}", chip);
            chip.draw_flag = false;
            window.draw_frame(&chip.graphics);
        }

        // Make sound
        beeper.set_beep(chip.make_sound);
    }
}
