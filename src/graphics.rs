use crate::cpu::CPU;
use piston_window::*;
use std::{thread, time};

pub struct GraphicsWindow {
    pub window: PistonWindow,
}

impl GraphicsWindow {
    pub fn new() -> GraphicsWindow {
        GraphicsWindow {
            window: WindowSettings::new("CHIP-8", [1920, 1080]).fullscreen(true)
            .exit_on_esc(true).build().unwrap(),
        }
    }

    pub fn draw(&mut self, mut cpu_mod: CPU) {
        let mut width: f64 = self.window.size().width;
        let mut pixel_size = width / 64.0;
        let mut events = self.window.events.ups(1);

        while let Some(e) = events.next(&mut self.window) {
            if let Some(_) = e.render_args() {
                self.window.draw_2d(&e, |c,g,_| {
                    //clear(color::BLACK, g);
                    cpu_mod.cycle();

                    for x in 0..64 {
                        for y in 0..32 {
                            let loc_x = x as f64 * pixel_size;
                            let loc_y = y as f64 * pixel_size;

                            if cpu_mod.gfx[x][y] == 1 {
                                let rect = Rectangle::new(color::WHITE);
                                let dims = rectangle::square(loc_x, loc_y, pixel_size);
                                rect.draw(dims, &draw_state::DrawState::default(), c.transform, g);
                            } else if cpu_mod.gfx[x][y] == 0 {
                                let rect = Rectangle::new(color::BLACK);
                                let dims = rectangle::square(loc_x, loc_y, pixel_size);
                                rect.draw(dims, &draw_state::DrawState::default(), c.transform, g);
                            }
                        }
                    } 
                });     
            }
        }
    }
}