use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use rand;



pub struct CPU {
    pub opcode: u16, // operation code;
    pub memory: [u8; 4096], // memory..duh;
    pub v: [u8; 16], // V registers that allow manipulation of data;
    pub i: usize, // 
    pub pc: usize, // program counter;
    pub gfx: [[u8; 32]; 64], // graphics array that maps out pixels on or off;
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    pub key: [u8; 16],
    pub fontset: [u16; 80]
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; 32]; 64],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            fontset: [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                      0x20, 0x60, 0x20, 0x20, 0x70, // 1
                      0xF0, 0x10, 0xF0, 0x10, 0x10, // 2
                      0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                      0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                      0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                      0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                      0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                      0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                      0xF0, 0x90 ,0xF0 ,0x10, 0xF0, // 9
                      0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                      0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                      0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                      0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                      0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                      0xF0, 0x80, 0xF0, 0x80, 0x80] // F
        }
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; 32]; 64],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            fontset: [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                      0x20, 0x60, 0x20, 0x20, 0x70, // 1
                      0xF0, 0x10, 0xF0, 0x10, 0x10, // 2
                      0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                      0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                      0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                      0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                      0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                      0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                      0xF0, 0x90 ,0xF0 ,0x10, 0xF0, // 9
                      0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                      0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                      0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                      0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                      0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                      0xF0, 0x80, 0xF0, 0x80, 0x80] // F
        }
    }

    pub fn process_opcode(&mut self, _opcode: u16) {
        self.opcode = _opcode;
        let mut op: [usize; 4] = [0; 4];

        // println!("0x{:x}", _opcode);

        // Split opcode into a slice:
        op[0] = ((self.opcode & 0xF000) >> 12) as usize;
        op[1] = ((self.opcode & 0x0F00) >>  8) as usize;
        op[2] = ((self.opcode & 0x00F0) >>  4) as usize;
        op[3] = ( self.opcode & 0x000F)        as usize;

        self.pc += 2;

        // Top of the stack is the current sp (stack pointer variable). This is the topmost part of the stack;

        match op[0] {
            0  => 
                match op[1] {
                    0 => 
                        match (op[2], op[3]) {
                            (14,  0) => { // Clear screen;
                                for x in 0..64 {
                                    for y in 0..32 {
                                        self.gfx[x][y] = 0;
                                    }
                                }
                            },
                            (14, 14) => {  // Return from a subroutine;
                                self.pc = self.stack[self.sp] as usize;
                                self.sp -= 1;
                            },
                            _        => println!("Execute machine language subroutine at adress {}{}{}", op[1], op[2], op[3]), // This instruction is ignored due to the fact that is only used on older machine computers;
                        },

                    _ => println!("No equivalent operation for supplied opcode!!"),
                },
            1  => self.pc = (self.opcode & 0x0FFF) as usize, // Jump to location nnn;
            2  => { // Call subroutine at nnn;
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = (self.opcode & 0x0FFF) as usize;
            }, 
            3  => { // Skip the next instruction if Vx == kk;
                if self.v[op[1]] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            },
            4  => { // Skip the next instruction if Vx != kk;
                if self.v[op[1]] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            },
            5  => { // Skip the next instruction if Vx == Vy;
                if self.v[op[1]] == self.v[op[2]] {
                    self.pc += 2;
                }
            },
            6  => { // Set Vx == kk;
                self.v[op[1]] = (self.opcode & 0x00FF) as u8;
            },
            7  => { // Set Vx = (Vx + kk);
                let kk = (self.opcode & 0x00FF) as u8;
                let added = u8::overflowing_add(self.v[op[1]], kk);

                self.v[op[1]] = added.0;
            },
            8  => 
                match op[3] {
                    0  => { // Set Vx = Vy;
                        self.v[op[1]] = self.v[op[2]];
                    },
                    1  => { // Set Vx = Vy OR Vy;
                        self.v[op[1]] = self.v[op[1]] | self.v[op[2]]; 
                    },
                    2  => { // Set Vx = Vx AND Vy;
                        self.v[op[1]] = self.v[op[1]] & self.v[op[2]];
                    },
                    3  => { // Set Vx = Vx XOR Vy;
                        self.v[op[1]] = self.v[op[1]] ^ self.v[op[2]];
                    },
                    4  => { // Set Vx = Vx + Vy, set VF = carry;
                        let (sum, carry) = self.v[op[1]].overflowing_add(self.v[op[2]]);
                        self.v[op[1]] = sum;

                        match carry {
                            true  => self.v[15] = 0,
                            false => self.v[15] = 1,
                        }
                    },
                    5  => { // Set Vx = Vx - Vy, set VF = carry;
                        let (diff, carry) = self.v[op[1]].overflowing_sub(self.v[op[2]]);
                        self.v[op[1]] = diff;

                        match carry {
                            true  => self.v[15] = 0,
                            false => self.v[15] = 1,
                        }
                    },
                    6  => { // Set Vx = Vx >> 1;
                        self.v[op[1]] = self.v[op[1]] >> 1;
                        self.v[15] = self.v[op[1]] & 0x1;
                        self.v[op[1]] = self.v[op[1]] / 2;
                    },
                    7  => { // Set Vx = Vy - Vx, set VF = NOT borrow;
                        let (diff, carry) = self.v[op[2]].overflowing_sub(self.v[op[1]]);
                        self.v[op[1]] = diff;

                        match carry {
                            true  => self.v[15] = 0,
                            false => self.v[15] = 1,
                        }
                    },
                    14 => { // Set Vx = Vx << 1;
                        self.v[op[1]] = self.v[op[1]] << 1;
                        self.v[15] = self.v[op[1]] & 0x80;
                    },
                    _  => println!("No equivalent operation for supplied opcode!"),
                },
            9  => { // Skip next instruction if Vx != Vy;
                if self.v[op[1]] != self.v[op[2]] {
                    self.pc += 2;
                }
            },
            10 => { // Set I = nnn;
                self.i = (self.opcode & 0x0FFF) as usize;
            },
            11 => { // Jump to location nnn + V0;5
                self.pc = ((self.opcode & 0x0FFF) + (self.v[0]) as u16) as usize;
            },
            12 => { // Set Vx = random byte AND kk;
                self.v[op[1]] = random_byte() & (self.opcode & 0x0FF) as u8;
            },
            13 => { // Display n-byte sprite starting at memory location I set at (Vx, Vy), and then set VF = collision;
                for y in 0..op[3] {
                    let pixel = self.read_mem(self.i + y);

                    for x in 0..4 {
                        if pixel.0[x] == 1 {
                            let loc_x = ((self.v[op[1]] + x as u8) % 64) as usize;
                            let loc_y = ((self.v[op[2]] + y as u8) % 32) as usize;

                            self.gfx[loc_x][loc_y] = pixel.0[x] ^ self.gfx[loc_x][loc_y];

                            if self.gfx[loc_x][loc_y] == 0 {
                                self.v[15] = 1;
                            }
                        }

                        if pixel.1[x] == 1 {
                            let loc_x = ((self.v[op[1]] + ((x + 4) % 4) as u8) % 64) as usize;
                            let loc_y = ((self.v[op[2]] + ((y + 4) % 4) as u8) % 32) as usize;

                            self.gfx[loc_x][loc_y] = pixel.1[x] ^ self.gfx[loc_x][loc_y];

                            if self.gfx[loc_x][loc_y] == 0 {
                                self.v[15] = 1;
                            }
                        }
                    }
                }
            },
            14 => 
                match op[2] {
                    9  => { // Skip next instruction if key with the value of Vx is pressed;
                        if self.key[self.v[op[1]] as usize] == 1 {
                            self.pc += 2;
                        }
                    },
                    10 => { // Skip next instruction if key with the value of Vx is not pressed;
                        if self.key[self.v[op[1]] as usize] == 0 {
                            self.pc += 2;
                        }
                    },
                    _  => println!("No equivalent operation for supplied opcode!")
                },
            15 =>
                match (op[2], op[3]) {
                    (0,  7) => { // Set Vx = delay timer value;
                        self.v[op[1]] = self.delay_timer; 
                    },
                    (0, 10) => { // Wait for a keypress, store the value of the key in Vx;
                        self.pc -= 2;

                        for i in 0..self.key.len() {
                            if self.key[i] == 1 {
                                self.pc += 2;
                                self.v[op[1]] = self.key[i];
                            }
                        }
                    },
                    (1,  5) => { // Set delay timer = Vx;
                        self.delay_timer = self.v[op[1]];
                    },
                    (1,  8) => { // Set sound timer = Vx;
                        self.sound_timer = self.v[op[1]];
                    },
                    (1, 14) => { // Set I = I + Vx;
                        self.i = self.i + self.v[op[1]] as usize;
                    },
                    (2,  9) => { // Set I = location of sprite for digit Vx;
                        self.i = self.v[op[1]] as usize;
                    },
                    (3,  3) => { // Store BCD representation of Vx in memory locations I, I + 1, and I + 2;
                        self.i = self.v[op[1]] as usize / 100;
                        self.i = (self.v[op[1] + 1] as usize / 10) % 10;
                        self.i = (self.v[op[1] + 2] as usize % 100) % 10;
                    },
                    (5,  5) => { // Store registers V0 -> Vx in memory starting at location I;
                        let mut _i = self.i;
                        for n in 0..op[1] {
                            self.memory[_i] = self.v[n];
                            _i += 1;
                        }
                    },
                    (6,  5) => { // Read registers V0 -> Vx from memory starting at location I;
                        let mut _i = self.i;
                        for n in 0..op[1] {
                            self.v[n] = self.memory[_i];
                            _i += 1;
                        }
                    },
                    _       => println!("No equivalent operation for supplied opcode!")
                }
            _  => println!("No opcode!"),
        }
    }

    pub fn cycle() {
        
    }

    pub fn read_mem(&mut self, index: usize) -> ([u8; 4], [u8; 4]) {
        let mut results = ([0; 4], [0; 4]);
        let nibble_1 = self.memory[index] & 0xF;
        let nibble_2 = self.memory[index] & 0x0F << 1;

        match nibble_1 {
            0  => results.0 = [0, 0, 0, 0],
            1  => results.0 = [0, 0, 0, 1],
            2  => results.0 = [0, 0, 1, 0],
            3  => results.0 = [0, 0, 1, 1],
            4  => results.0 = [0, 1, 0, 0],
            5  => results.0 = [0, 1, 0, 1],
            6  => results.0 = [0, 1, 1, 0],
            7  => results.0 = [0, 1, 1, 1],
            8  => results.0 = [1, 0, 0, 0],
            9  => results.0 = [1, 0, 0, 1],
            10 => results.0 = [1, 0, 1, 0],
            11 => results.0 = [1, 0, 1, 1],
            12 => results.0 = [1, 1, 0, 0],
            13 => results.0 = [1, 1, 0, 1],
            14 => results.0 = [1, 1, 1, 0],
            15 => results.0 = [1, 1, 1, 1],
            _  => eprintln!("Invalid memory nibble! {}", nibble_1)
        }

        match nibble_2 {
            0  => results.1 = [0, 0, 0, 0],
            1  => results.1 = [0, 0, 0, 1],
            2  => results.1 = [0, 0, 1, 0],
            3  => results.1 = [0, 0, 1, 1],
            4  => results.1 = [0, 1, 0, 0],
            5  => results.1 = [0, 1, 0, 1],
            6  => results.1 = [0, 1, 1, 0],
            7  => results.1 = [0, 1, 1, 1],
            8  => results.1 = [1, 0, 0, 0],
            9  => results.1 = [1, 0, 0, 1],
            10 => results.1 = [1, 0, 1, 0],
            11 => results.1 = [1, 0, 1, 1],
            12 => results.1 = [1, 1, 0, 0],
            13 => results.1 = [1, 1, 0, 1],
            14 => results.1 = [1, 1, 1, 0],
            15 => results.1 = [1, 1, 1, 1],
            _  => eprintln!("Invalid memory nibble! {}", nibble_2)
        }

        results
    }

    pub fn read_rom(&mut self) {
        let _opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);

        self.process_opcode(_opcode);
    }

    pub fn load_rom(&mut self, name: String) {
        let dir = env::current_dir().unwrap();
        let path: String = format!("{}{}{}{}", dir.display(), "\\roms\\", name, ".ch8");
        
        let file = File::open(path).unwrap();

        let file_rom: Vec<u8> = file.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
    
        // Make sure that the program counter is set to 0x200 (512);
        let mut _pc = 0x200;

        for i in 0..file_rom.len() {
            self.memory[_pc] = file_rom[i];
            _pc += 1;
        }
    }
}

fn random_byte() -> u8 {
    rand::random::<u8>()
}