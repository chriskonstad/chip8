extern crate chip8;
extern crate sdl2;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::Sdl;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub struct Display<'a> {
    width: u32,
    height: u32,
    renderer: sdl2::render::Renderer<'a>,
    texture: sdl2::render::Texture,
    frame_duration: Duration,
    frame_last: Instant,
}

impl<'a> Display<'a> {
    pub fn new(context: &Sdl,
               title: &str,
               width: u32,
               height: u32,
               duration: Duration) -> Self {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut renderer = window.renderer().build().unwrap();
        let mut texture = renderer.create_texture_streaming(
            PixelFormatEnum::RGB24, 64, 32).unwrap();

        Display {
            width: width,
            height: height,
            renderer: renderer,
            texture: texture,
            frame_duration: duration,
            frame_last: Instant::now(),
        }
    }

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

        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..32 {
                for x in 0..64 {
                    let offset = y*pitch + x*3;
                    let value = if 0 != bitmap[y * 64 + x] { 255 } else { 0 };
                    buffer[offset + 0] = value as u8;
                    buffer[offset + 1] = value as u8;
                    buffer[offset + 2] = 0;
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

