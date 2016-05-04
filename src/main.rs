extern crate libchip8;
extern crate sdl2;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::time::{Duration,Instant};
use std::thread::sleep;
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use libchip8::Chip8;

const scale : u32 = 8;
const width : u32 = 64 * scale;
const height : u32 = 32 * scale;

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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Chip8 Emulator", width, height)
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
        println!("Last frame: {:?}", last_frame);
        if last_frame < one_frame {
            let diff = one_frame - last_frame;
            println!("Sleeping for: {:?}", diff);
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
        chip.emulateCycle();

        if chip.drawFlag {
            // TODO
            // drawGraphics();
            print!("{:?}", chip);
            chip.drawFlag = false;
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
            renderer.copy(&texture, None, Some(Rect::new(0, 0, width, height)));
            renderer.present();
        }

        // Store key press state
        //chip.setKeys();
    }
}
