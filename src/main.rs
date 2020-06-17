mod cpu;

fn main() {
    let mut cpu_mod = cpu::CPU::new();
    cpu_mod.dec_to_hex(12);
    //let results = ((1 << 12) & 0xF000) | ((4 << 8) & 0x0F00) | ((2 << 4) & 0x00F0) | (4 & 0x000F);
    // cpu_mod.opcode = results;
    // cpu_mod.process_opcode();
    
}
