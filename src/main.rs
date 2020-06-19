mod cpu;

fn main() {
    let mut cpu_mod = cpu::CPU::new();
    cpu_mod.v[0xA] = 60;
    cpu_mod.process_opcode(0xDA2F);
    //let results = ((1 << 12) & 0xF000) | ((4 << 8) & 0x0F00) | ((2 << 4) & 0x00F0) | (4 & 0x000F);
    // cpu_mod.opcode = results;
    // cpu_mod.process_opcode();
    
}
