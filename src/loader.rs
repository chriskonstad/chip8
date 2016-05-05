use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Loads the given filename as a vetor of bytes.
pub fn load_file(path: &str) -> Vec<u8> {
    // Initialize the emulator and load the game
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut game = Vec::new();
    match file.read_to_end(&mut game) {
        Err(why) => panic!("Couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    };

    game
}
