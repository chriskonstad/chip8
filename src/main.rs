extern crate chip8;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;

mod audio;
mod graphics;

use chip8::Chip8;
use clap::{Arg, App};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, KeyboardState, Scancode};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

const SCALE : u32 = 8;
const WIDTH : u32 = 64 * SCALE;
const HEIGHT : u32 = 32 * SCALE;

fn check_keys(chip : &mut Chip8, kb : &KeyboardState) {
    chip.key[0x0] = kb.is_scancode_pressed(Scancode::Num0) as u8;
    chip.key[0x1] = kb.is_scancode_pressed(Scancode::Num1) as u8;
    chip.key[0x2] = kb.is_scancode_pressed(Scancode::Num2) as u8;
    chip.key[0x3] = kb.is_scancode_pressed(Scancode::Num3) as u8;
    chip.key[0x4] = kb.is_scancode_pressed(Scancode::Num4) as u8;
    chip.key[0x5] = kb.is_scancode_pressed(Scancode::Num5) as u8;
    chip.key[0x6] = kb.is_scancode_pressed(Scancode::Num6) as u8;
    chip.key[0x7] = kb.is_scancode_pressed(Scancode::Num7) as u8;
    chip.key[0x8] = kb.is_scancode_pressed(Scancode::Num8) as u8;
    chip.key[0x9] = kb.is_scancode_pressed(Scancode::Num9) as u8;
    chip.key[0xA] = kb.is_scancode_pressed(Scancode::A) as u8;
    chip.key[0xB] = kb.is_scancode_pressed(Scancode::B) as u8;
    chip.key[0xC] = kb.is_scancode_pressed(Scancode::C) as u8;
    chip.key[0xD] = kb.is_scancode_pressed(Scancode::D) as u8;
    chip.key[0xE] = kb.is_scancode_pressed(Scancode::E) as u8;
    chip.key[0xF] = kb.is_scancode_pressed(Scancode::F) as u8;
}

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

    // Setup the audio
    let mut beeper = audio::Beeper::new(&sdl_context,
                                        Duration::from_millis(250));

    // Setup the input
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Emulation loop
    'running: loop {
        // Handle quit event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Run a cycle on the chip
        chip.emulate_cycle();

        // Render the frame if needed
        if chip.draw_flag {
            debug!("{:?}", chip);
            chip.draw_flag = false;
            window.draw_frame(&chip.graphics);
        }

        // Store key press state
        let keyboard_state = KeyboardState::new(&event_pump);
        check_keys(&mut chip, &keyboard_state);

        // Make sound
        beeper.set_beep(chip.make_sound);
    }
}
