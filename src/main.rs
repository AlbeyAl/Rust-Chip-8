struct CPU {
    opcode: u16, // operation code;
    memory: [u8; 4096], // memory..duh;
    v: [u8; 16], // V registers that allow manipulation of data;
    i: u16, // 
    pc: u16, // program counter;
    gfx: [[u8; 64]; 32], // graphics array that maps out pixels on or off;
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16]
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

fn main() {
    
}
