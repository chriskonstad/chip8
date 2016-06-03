extern crate chip8;
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::Sdl;
use std::time::{Duration, Instant};
use std::thread::sleep;

/// Represents a color to paint the display with
#[derive(RustcDecodable, RustcEncodable)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Represents a display.  In this case, it represents an SDL window.
pub struct Display<'a> {
    width: u32,
    height: u32,
    renderer: sdl2::render::Renderer<'a>,
    texture: sdl2::render::Texture,
    frame_duration: Duration,
    frame_last: Instant,
    color: Color,
}

impl<'a> Display<'a> {
    /// Constructs a new SDL window with the given SDL context,
    /// the given title, the given width (in pixels, the given height
    /// (in pixels), and the given frame duration.
    ///
    /// The frame duration is used to set how long each frame should last.
    /// Higher values lead to slower emulator performance but also help
    /// smooth the framerate.
    pub fn new(context: &Sdl,
               title: &str,
               width: u32,
               height: u32,
               duration: Duration,
               color: Color) -> Self {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer().build().unwrap();
        let texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, 64, 32).unwrap();

        Display {
            width: width,
            height: height,
            renderer: renderer,
            texture: texture,
            frame_duration: duration,
            frame_last: Instant::now(),
            color: color,
        }
    }

    /// The window draws the give bitmap image on the Display's frame duration.
    ///
    /// If there is time left over, that time is spent sleeping.
    pub fn draw_frame(&mut self, bitmap: &[u8; chip8::NPIXELS]) {
        // Keep timing okay
        let prev_duration = Instant::now().duration_since(self.frame_last);
        debug!("Last frame duration: {:?}", prev_duration);
        if prev_duration < self.frame_duration {
            let diff = self.frame_duration - prev_duration;
            debug!("Sleeping for: {:?}", diff);
            sleep(diff);
        }
        self.frame_last = Instant::now();

        let color = &self.color;
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..32 {
                for x in 0..64 {
                    let offset = y*pitch + x*3;
                    let enabled = if 0 != bitmap[y * 64 + x] { 1 } else { 0 };
                    buffer[offset + 0] = enabled * color.red;
                    buffer[offset + 1] = enabled * color.green;
                    buffer[offset + 2] = enabled * color.blue;
                }
            }

        }).unwrap();
        self.renderer.clear();
        self.renderer.copy(&self.texture,
                           None,
                           Some(Rect::new(0, 0, self.width, self.height)));
        self.renderer.present();
    }
}

