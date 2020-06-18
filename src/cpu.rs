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

    pub fn process_opcode(&mut self) {
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
                    4  => { // Set Vx = Vx + Vy, set VF = carry; This code block was brilliantly written by ColinEberhardt (https://github.com/ColinEberhardt/wasm-rust-chip8); 
                        let (sum, carry) = self.v[op[1]].overflowing_add(self.v[op[2]]);
                        self.v[op[1]] = sum;

                        match carry {
                            true  => self.v[15] = 0,
                            false => self.v[15] = 1,
                        }
                    },
                    5  => { // Set Vx = Vx - Vy, set VF = carry; This code block was brilliantly written by ColinEberhardt (https://github.com/ColinEberhardt/wasm-rust-chip8);
                        let (diff, carry) = self.v[op[1]].overflowing_sub(self.v[op[2]]);
                        self.v[op[1]] = diff;

                        match carry {
                            true  => self.v[15] = 0,
                            false => self.v[15] = 1,
                        }
                    },
                    6  => { // Set Vx = Vx >> 1; This code block was brilliantly written by ColinEberhardt (https://github.com/ColinEberhardt/wasm-rust-chip8);
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
            9  => println!("Skip the following instruction if the value of register VX != VY"),
            10 => println!("Store memory address NNN in register index I"),
            11 => println!("Jump to address NNN + V0"),
            12 => println!("Set VX to a random number with a mask of NN"),
            13 => println!("Draw a sprite at location VX, VY with N bytes of sprite data starting at the address stored in I"),
            14 => 
                match op[2] {
                    9  => println!("Skip the following instruction if the key corresponding to the hex value currently stored in register VX is pressed"),
                    10 => println!("Skip the following instruction if the key corresponding to the hex value currrently stored in register VX is not pressed"),
                    _  => println!("No equivalent operation for supplied opcode!")
                },
            15 =>
                match (op[2], op[3]) {
                    (0,  7) => println!("Store the current value of the delay timer in register VX"),
                    (0, 10) => println!("Wait for a keypress and store the result in register VX"),
                    (1,  5) => println!("Set the delay timer to the value of reigster VX"),
                    (1,  8) => println!("Set the sound timer to the value of register VX"),
                    (1, 14) => println!("Add the value stored in register VX to register index I"),
                    (2,  9) => println!("Set I to the memory address of the sprite data corresponding to the hexadecimal digit stored in register VX"),
                    (3,  3) => println!("Store the binary-coded decimal equivalent of the value stored in register VX at addresses I, I + 1, and I + 2"),
                    (5,  5) => println!("Store the values of registers V0 to VX inclusive in memory starting at address I\n
                                            I is set to I + X + 1 after operation"),
                    (6,  5) => println!("Fill registers V0 to VX inclusive with the values stored in memory at address I\n
                                            I is set to I + X + 1 after operation"),
                    _       => println!("No equivalent operation for supplied opcode!")
                }
            _  => println!("No opcode!"),
        }
    }

    pub fn dec_to_hex(&mut self, decimal: u16) -> (){
        let mut quotient = decimal;
        let mut remainder: u16;
        let mut hexadecimal: Vec<u16> = Vec::new();

        while quotient != 0 {
            remainder = quotient % 16;

            hexadecimal.push(remainder);

            quotient = quotient / 16;
        }

        // not entirely sure how to return the vec! as a usize;
    }
}