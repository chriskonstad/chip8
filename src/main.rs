extern crate libchip8;
extern crate sdl2;

use libchip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, KeyboardState, Scancode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration, Instant};

const SCALE : u32 = 8;
const WIDTH : u32 = 64 * SCALE;
const HEIGHT : u32 = 32 * SCALE;

fn check_keys(chip : &mut Chip8, kb : &KeyboardState) {
    chip.key[0x0] = if kb.is_scancode_pressed(Scancode::Num0) { 1 } else { 0 };
    chip.key[0x1] = if kb.is_scancode_pressed(Scancode::Num1) { 1 } else { 0 };
    chip.key[0x2] = if kb.is_scancode_pressed(Scancode::Num2) { 1 } else { 0 };
    chip.key[0x3] = if kb.is_scancode_pressed(Scancode::Num3) { 1 } else { 0 };
    chip.key[0x4] = if kb.is_scancode_pressed(Scancode::Num4) { 1 } else { 0 };
    chip.key[0x5] = if kb.is_scancode_pressed(Scancode::Num5) { 1 } else { 0 };
    chip.key[0x6] = if kb.is_scancode_pressed(Scancode::Num6) { 1 } else { 0 };
    chip.key[0x7] = if kb.is_scancode_pressed(Scancode::Num7) { 1 } else { 0 };
    chip.key[0x8] = if kb.is_scancode_pressed(Scancode::Num8) { 1 } else { 0 };
    chip.key[0x9] = if kb.is_scancode_pressed(Scancode::Num9) { 1 } else { 0 };
    chip.key[0xA] = if kb.is_scancode_pressed(Scancode::A) { 1 } else { 0 };
    chip.key[0xB] = if kb.is_scancode_pressed(Scancode::B) { 1 } else { 0 };
    chip.key[0xC] = if kb.is_scancode_pressed(Scancode::C) { 1 } else { 0 };
    chip.key[0xD] = if kb.is_scancode_pressed(Scancode::D) { 1 } else { 0 };
    chip.key[0xE] = if kb.is_scancode_pressed(Scancode::E) { 1 } else { 0 };
    chip.key[0xF] = if kb.is_scancode_pressed(Scancode::F) { 1 } else { 0 };
}

fn main() {
    println!("Chip8 emulator in Rust");

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
    chip.load_hex(&game);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip8 Emulator", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, 64, 32).unwrap();

    // TODO TEST AT 60frames a second!
    // This isn't set to 60Hz because that was too slow
    let one_frame = Duration::from_millis(1);
    let mut current_time = Instant::now();

    // Emulation loop
    'running: loop {
        let last_frame = Instant::now().duration_since(current_time);
        //println!("Last frame: {:?}", last_frame);
        if last_frame < one_frame {
            let diff = one_frame - last_frame;
            //println!("Sleeping for: {:?}", diff);
            sleep(diff);
        }
        current_time = Instant::now();

        // Handle quit event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        chip.emulate_cycle();

        if chip.draw_flag {
            //print!("{:?}", chip);
            chip.draw_flag = false;
            texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..32 {
                    for x in 0..64 {
                        let offset = y*pitch + x*3;
                        let value = if 0 != chip.graphics[y * 64 + x] { 255 } else { 0 };
                        buffer[offset + 0] = value as u8;
                        buffer[offset + 1] = value as u8;
                        buffer[offset + 2] = 0;
                    }
                }

            }).unwrap();
            renderer.clear();
            renderer.copy(&texture, None, Some(Rect::new(0, 0, WIDTH, HEIGHT)));
            renderer.present();
        }

        // Store key press state
        let keyboard_state = KeyboardState::new(&event_pump);
        check_keys(&mut chip, &keyboard_state);

        // Make sound
        if chip.make_sound {
            unsafe {
                println!("{}", std::char::from_u32_unchecked(7));
            }
        }
    }
}
