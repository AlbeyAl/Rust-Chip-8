use rand::Rng;

pub struct CPU {
    pub opcode: u16, // operation code;
    pub memory: [u8; 4096], // memory..duh;
    pub v: [u8; 16], // V registers that allow manipulation of data;
    pub i: usize, // 
    pub pc: usize, // program counter;
    pub gfx: [[u8; 64]; 32], // graphics array that maps out pixels on or off;
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
    pub key: [u8; 16]
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16]
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
            gfx: [[0; 64]; 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16]
        }
    }

    pub fn process_opcode(&mut self, _opcode: u16) {
        self.opcode = _opcode;
        let mut op: [usize; 4] = [0; 4];

        // Split opcode into a slice:
        op[0] = ((self.opcode & 0xF000) >> 12) as usize;
        op[1] = ((self.opcode & 0x0F00) >>  8) as usize;
        op[2] = ((self.opcode & 0x00F0) >>  4) as usize;
        op[3] = ( self.opcode & 0x000F)        as usize;

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
                self.v[op[1]] = self.v[op[1]] + (self.opcode & 0x00FF) as u8; // <-- test use -->
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
            11 => { // Jump to location nnn + V0;
                self.pc = ((self.opcode & 0x0FFF) + (self.v[0]) as u16) as usize;
            },
            12 => { // Set Vx = random byte AND kk;
                self.v[op[1]] = random_byte() & (self.opcode & 0x0FF) as u8;
            },
            13 => { // Display n-byte sprite starting at memory location I set at (Vx, Vy), and then set VF = collision;
                for i in 0..op[3] {
                    let pixels: Vec<u8> = Vec::new();

                    for j in 0..8 {
                        let _i: usize = i;
                        let _j: usize = j;

                        let x = (_j + self.v[op[1]] as usize) % 64;
                        let y = (_i + self.v[op[2]] as usize) % 32;

                        if pixels[_j] == 1 {
                            self.gfx[x][y] = pixels[_j] ^ self.gfx[x][y];
                            if self.gfx[x][y] == 0 { self.v[15] = 1 } else { self.v[15] = 0 }
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
}

fn random_byte() -> u8 {
    rand::random::<u8>()
}