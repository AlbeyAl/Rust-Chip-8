pub struct CPU {
    pub opcode: u16, // operation code;
    pub memory: [u8; 4096], // memory..duh;
    pub v: [u8; 16], // V registers that allow manipulation of data;
    pub i: u16, // 
    pub pc: u16, // program counter;
    pub gfx: [[u8; 64]; 32], // graphics array that maps out pixels on or off;
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: u16,
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
        let mut op: [u16; 4] = [0; 4];

        // Split opcode into a slice:
        op[0] = (self.opcode & 0xF000) >> 12;
        op[1] = (self.opcode & 0x0F00) >>  8;
        op[2] = (self.opcode & 0x00F0) >>  4;
        op[3] =  self.opcode & 0x000F       ;

        println!("Opcode: {}{}{}{}", (op[0]) as usize, (op[1]) as usize, (op[2]) as usize, (op[3]) as usize);

        match op[0] {
            0  => 
                match op[1] {
                    0 => 
                        match op[3] {
                            0 => println!("Clear the screen"),
                            14 => println!("Return from a subroutine"),
                            _ => println!("Execute machine language subroutine at adress {}{}{}", op[1], op[2], op[3]),
                        },

                    _ => println!("Execute machine language subroutine at adress {}{}{}", op[1], op[2], op[3]),
                },
            1  => println!("Jump to adress NNN"),
            2  => println!("Execute subroutine starting at address NNN"),
            3  => println!("Skip the following instruction if the value of register VX == NN"),
            4  => println!("Skip the following instruction if the value of register VX != NN"),
            5  => println!("Skip the following instruction if the value of register VX is equal to the value of register VY"),
            6  => println!("Store number NN in register VX"),
            7  => println!("Add the value NN to register VX"),
            8  => 
                match op[3] {
                    0  => println!("Store the value of register VY in register VX"),
                    1  => println!("Set VX to VX /OR/ VY"),
                    2  => println!("Set VX to VX /AND/ VY"),
                    3  => println!("Set VX to VX /XOR/ VY"),
                    4  => println!("Add the value of register VY to register VX\n
                                    Set VF to 01 if a carry occurs\n
                                    Set VF to 00 if a carry doesn't occur"),
                    5  => println!("Subtract the value of register VY from register VX\n
                                    Set VF to 00 if a borrow occurs\n
                                    Set VF to 01 if a borrow doesn't occur"),
                    6  => println!("Store the value of register VY shifted right one bit in register VX\n
                                    Set register VF to the least significant bit prior to the shift\n
                                    VY is unchanged"),
                    7  => println!("Set register VX to the value of VY minus VX\n
                                    Set VF to 00 if a borrow occurs\n
                                    Set VF to 01 if a borrow doesn't occur"),
                    14 => println!("Store the value of register VY shifted left one bit in register VX\n
                                    Set register VF to the most significant bit prior the shift\n
                                    VY is unchanged"),
                    _  => println!("No equivalent operation for supplied opcode"),
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
                    _  => println!("No equivalent operation for supplied opcode")
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
                    _       => println!("No equivalent operation for supplied opcode")
                }
            _  => println!("No opcode!"),
        }

    }
}