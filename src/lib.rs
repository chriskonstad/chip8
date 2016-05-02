use std::mem;
use std::num::Wrapping;
use std::vec::Vec;

const NMEM : usize = 4096;
const NREG : usize = 16;
const NPIXELS : usize = 64 * 32;

/*
 * From http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
 * MEMORY MAP:
 * 0x000-0x1FF: Chip 8 interpreter
 * 0x050-0x0A0: 4x5 pixel font set (0-F)
 * 0x200-0xFFF: Program ROM and RAM
 */
static FONTSET : [u8;80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    drawFlag: bool,
    opcode: u16,
    memory: [u8; NMEM],
    reg: [u8; NREG],
    index: u16,
    pc: u16,
    graphics: [u8; NPIXELS],
    timer_delay: u8,
    timer_sound: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 {
            drawFlag: false,
            opcode: 0,
            memory: [0; NMEM],
            reg: [0; NREG],
            index: 0,
            pc: 0x200,
            graphics: [0; NPIXELS],
            timer_delay: 0,
            timer_sound: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
        };

        // Initialize the font set
        for i in 0..80 {
            chip.memory[i] = FONTSET[i];
        }

        chip
    }

    pub fn loadHex(&mut self, game: &Vec<u8>) {
        for c in 0..game.len() {
            self.memory[c + 512] = game[c];
        }
    }

    pub fn loadGame(&mut self, game: String) {
        // TODO Fill in memory starting at 0x200
        unimplemented!();
    }

    pub fn emulateCycle(&mut self) {
        // Fetch opcode
        self.fetchOpcode();

        // Decode and Execute opcode
        self.executeOpcode();

        // Update timers
        self.updateTimers();
    }

    pub fn fetchOpcode(&mut self) {
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
    }

    pub fn executeOpcode(&mut self) {
        // TODO Fill in table
        println!("Opcode: {:#X}", self.opcode);
        match self.opcode & 0xF000 {
            0x1000 => {
                // 0x1NNN: Jump to address NNN
                self.pc = self.opcode & 0x0FFF;
            }
            0x2000 => {
                // 0x2NNN: Call subroutine at NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            },
            0x3000 => {
                // 0x3XNN: Skip next instruction if regX equals NN
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = (self.opcode & 0x00FF) as u8;
                if self.reg[x as usize] == nn {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x4000 => {
                // 0x4XNN: Skip next instruction if regX does not equal NN
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = (self.opcode & 0x00FF) as u8;
                if self.reg[x as usize] != nn {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x5000 => {
                // 0x5XY0: Skip next instruction if regX equals regY
                let x = (self.opcode & 0x0F00) >> 8;
                let y = (self.opcode & 0x00F0) >> 4;
                if self.reg[x as usize] == self.reg[y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            },
            0x6000 => {
                // 0x6XNN: Set regX to NN
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = (self.opcode & 0x00FF) as u8;
                self.reg[x as usize] = nn;
                self.pc += 2;
            },
            0x7000 => {
                // 0x7XNN: Add NN to regX
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = (self.opcode & 0x00FF) as u8;
                self.reg[x as usize] += nn;
                self.pc += 2;
            },
            0x8000 => {
                match self.opcode & 0x000F {
                    0x0000 => {
                        // 0x8XY0: Set regX to regY
                        let x = (self.opcode & 0x0F00) >> 8;
                        let y = (self.opcode & 0x00F0) >> 4;
                        self.reg[x as usize] = self.reg[y as usize];
                        self.pc += 2;
                    }
                    0x0001 => {
                        // 0x8XY1: Set regX to regX | regY
                        let x = (self.opcode & 0x0F00) >> 8;
                        let y = (self.opcode & 0x00F0) >> 4;
                        self.reg[x as usize] = self.reg[x as usize] | self.reg[y as usize];
                        self.pc += 2;
                    }
                    0x0002 => {
                        // 0x8XY2: Set regX to regX & regY
                        let x = (self.opcode & 0x0F00) >> 8;
                        let y = (self.opcode & 0x00F0) >> 4;
                        self.reg[x as usize] = self.reg[x as usize] & self.reg[y as usize];
                        self.pc += 2;
                    }
                    0x0003 => {
                        // 0x8XY3: Set regX to regX ^ regY
                        let x = (self.opcode & 0x0F00) >> 8;
                        let y = (self.opcode & 0x00F0) >> 4;
                        self.reg[x as usize] = self.reg[x as usize] ^ self.reg[y as usize];
                        self.pc += 2;
                    }
                    0x0004 => {
                        // 0x8XY4: Add regY to regX, set carry if needed
                        if self.reg[((self.opcode & 0x00F0) >> 4) as usize] >
                            (0xFF - self.reg[((self.opcode & 0x0F00) >> 8) as usize]) {
                            self.reg[0xF] = 1;
                        } else {
                            self.reg[0xF] = 0;
                        }
                        let x = Wrapping(self.reg[((self.opcode & 0x0F00) >> 8) as usize]);
                        let y = Wrapping(self.reg[((self.opcode & 0x00F0) >> 4) as usize]);
                        self.reg[((self.opcode & 0x0F00) >> 8) as usize] = (x + y).0;
                        self.pc += 2;
                    }
                    _ => panic!("Opcode {:#X} is bad", self.opcode),
                }
            },
            0xA000 => {
                // 0xANNN: Sets I to the address NNN
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            },
            0xB000 => {
                // 0xBNNN: Jump to address NNN + reg0
                let address = self.opcode & 0x0FFF;
                self.pc = address + self.reg[0] as u16;
            }
            _ => panic!("Opcode {:#X} is bad", self.opcode),
        }
    }

    pub fn updateTimers(&mut self) {
        if(self.timer_delay > 0) {
            self.timer_delay -= 1;
        }

        if(self.timer_sound > 0) {
            if(self.timer_sound == 1) {
                // TODO beep
                unimplemented!();
            }
            self.timer_sound -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Chip8;

    #[test]
    fn op1nnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x16, 0x66]);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.pc, 0x666);
    }

    #[test]
    fn op2nnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x26, 0x66]);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.pc, 0x666);
        assert_eq!(chip.stack[0], 512);
        assert_eq!(chip.sp, 1);
    }

    #[test]
    fn op3xnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x31, 0x66, 0x31, 0x67]);
        chip.reg[1] = 0x67;
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.pc, 514);
        chip.emulateCycle();
        assert_eq!(chip.pc, 518);
    }

    #[test]
    fn op4xnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x41, 0x66, 0x41, 0x67]);
        chip.reg[1] = 0x66;
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.pc, 514);
        chip.emulateCycle();
        assert_eq!(chip.pc, 518);
    }

    #[test]
    fn op5xy0() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x51, 0x20, 0x51, 0x30]);
        chip.reg[1] = 0x66;
        chip.reg[2] = 0x22;
        chip.reg[3] = 0x66;
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.pc, 514);
        chip.emulateCycle();
        assert_eq!(chip.pc, 518);
    }

    #[test]
    fn op6xnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x6A, 0x2F]);
        assert_eq!(chip.reg[0xA], 0);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0x2F);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op7xnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x7A, 0x2F]);
        chip.reg[0xA] = 0xB;
        assert_eq!(chip.reg[0xA], 0xB);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0x2F + 0xB);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op8xy0() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x8A, 0x20]);
        chip.reg[0xA] = 0xB;
        chip.reg[0x2] = 0xC;
        assert_eq!(chip.reg[0xA], 0xB);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0xC);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op8xy1() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x8A, 0x21]);
        chip.reg[0xA] = 0xB;
        chip.reg[0x2] = 0xC;
        assert_eq!(chip.reg[0xA], 0xB);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0xB | 0xC);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op8xy2() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x8A, 0x22]);
        chip.reg[0xA] = 0xB;
        chip.reg[0x2] = 0xC;
        assert_eq!(chip.reg[0xA], 0xB);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0xB & 0xC);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op8xy3() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x8A, 0x23]);
        chip.reg[0xA] = 0xB;
        chip.reg[0x2] = 0xC;
        assert_eq!(chip.reg[0xA], 0xB);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.reg[0xA], 0xB ^ 0xC);
        assert_eq!(chip.reg[0x2], 0xC);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn op8xy4() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0x8A, 0xB4, 0x8B, 0xC4]);
        chip.reg[0xA] = 0x00;
        chip.reg[0xB] = 0xFF;
        chip.reg[0xC] = 0x01;
        assert_eq!(chip.pc, 512);

        chip.emulateCycle();
        assert_eq!(chip.pc, 514);
        assert_eq!(chip.reg[0xA], 0xFF);
        assert_eq!(chip.reg[0xB], 0xFF);
        assert_eq!(chip.reg[0xF], 0x00);

        chip.emulateCycle();
        assert_eq!(chip.pc, 516);
        assert_eq!(chip.reg[0xB], 0x0);
        assert_eq!(chip.reg[0xC], 0x1);
        assert_eq!(chip.reg[0xF], 0x1);
    }

    #[test]
    fn opAnnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0xA6, 0x66]);
        assert_eq!(chip.index, 0);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.index, 0x666);
        assert_eq!(chip.pc, 514);
    }

    #[test]
    fn opBnnn() {
        let mut chip = Chip8::new();
        chip.loadHex(&vec![0xB6, 0x66]);
        chip.reg[0] = 0x5;
        assert_eq!(chip.index, 0);
        assert_eq!(chip.pc, 512);
        chip.emulateCycle();
        assert_eq!(chip.index, 0);
        assert_eq!(chip.pc, 0x666 + 0x5);
    }
}
