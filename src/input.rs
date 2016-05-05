extern crate sdl2;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::keyboard::{Keycode, KeyboardState, Scancode};

/// This struct keeps track of the SDL EventPump, which is used for scanning
/// the keyboard for key presses.
pub struct Keyboard {
    event_pump: EventPump,
}

/// This enum says whether or not the user is trying to quit.
pub enum Command {
    /// The caller should quit.
    Quit,
    /// The caller should continue running.
    Continue,
}

impl Keyboard {
    /// Constructs a new Keyboard from the given SDL context.
    pub fn new(context: &Sdl) -> Self {
        Keyboard {
            event_pump: context.event_pump().unwrap(),
        }
    }

    /// Checks the keyboard's keys, looking for quit events and which keys
    /// should be marked as pressed in the given key state array.
    pub fn check(&mut self, keys: &mut [u8; 16]) -> Command {
        // Handle quit event
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Command::Quit;
                },
                _ => {}
            }
        }

        // Record the keyboard state
        let kb = KeyboardState::new(&self.event_pump);
        keys[0x0] = kb.is_scancode_pressed(Scancode::Num0) as u8;
        keys[0x1] = kb.is_scancode_pressed(Scancode::Num1) as u8;
        keys[0x2] = kb.is_scancode_pressed(Scancode::Num2) as u8;
        keys[0x3] = kb.is_scancode_pressed(Scancode::Num3) as u8;
        keys[0x4] = kb.is_scancode_pressed(Scancode::Num4) as u8;
        keys[0x5] = kb.is_scancode_pressed(Scancode::Num5) as u8;
        keys[0x6] = kb.is_scancode_pressed(Scancode::Num6) as u8;
        keys[0x7] = kb.is_scancode_pressed(Scancode::Num7) as u8;
        keys[0x8] = kb.is_scancode_pressed(Scancode::Num8) as u8;
        keys[0x9] = kb.is_scancode_pressed(Scancode::Num9) as u8;
        keys[0xA] = kb.is_scancode_pressed(Scancode::A) as u8;
        keys[0xB] = kb.is_scancode_pressed(Scancode::B) as u8;
        keys[0xC] = kb.is_scancode_pressed(Scancode::C) as u8;
        keys[0xD] = kb.is_scancode_pressed(Scancode::D) as u8;
        keys[0xE] = kb.is_scancode_pressed(Scancode::E) as u8;
        keys[0xF] = kb.is_scancode_pressed(Scancode::F) as u8;

        Command::Continue
    }
}
