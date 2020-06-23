mod cpu;
mod graphics;
use std::{thread, time};

fn main() {
    let mut cpu_mod = cpu::CPU::new();
    let mut window = graphics::GraphicsWindow::new();

    cpu_mod.load_rom("Astro Dodge [Revival Studios, 2008]".to_string());
    //cpu_mod.load_rom("test_opcode".to_string());
    //cpu_mod.load_rom("Pong (1 player)".to_string());
    window.draw(cpu_mod);

    // loop {
    //     for _ in 0..60 {
    //         cpu_mod.cycle(&mut gfx);
    //     }

    //     thread::sleep(time::Duration::from_secs(1));
    // }

    // thread::spawn(move || {

    // });
    
    // loop {
    //     for _ in 0..60 {
    //         cpu_mod.cycle();
    //     }

    //     thread::sleep(time::Duration::from_secs(1));
    // }
}
