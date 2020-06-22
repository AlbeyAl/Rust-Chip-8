mod cpu;
use std::{thread, time};

fn main() {
    let mut cpu_mod = cpu::CPU::new();
    cpu_mod.load_rom("test_opcode".to_string());
    
    loop {
        cpu_mod.read_rom();
        // thread::sleep(time::Duration::from_millis(500));
    }
}
