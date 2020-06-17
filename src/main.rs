mod cpu;

fn main() {
    let mut cpu_mod = cpu::CPU::new();
    cpu_mod.opcode = 0x00FF;
    cpu_mod.process_opcode();
}
