use std::path::Path;

use minifb::{Key, Window, WindowOptions};

use crate::cpu::Cpu;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub struct Gameboy {
    pub cpu: Cpu,
}

impl Gameboy {
    pub fn new(rom_file: &Path) -> Self {
        Self {
            cpu: Cpu::new(rom_file),
        }
    }

    pub fn run(&mut self) {
        let mut window = Window::new("Rustyboy", WIDTH, HEIGHT, WindowOptions::default())
            .unwrap_or_else(|e| panic!("{}", e));
        let buffer = vec![125; WIDTH * HEIGHT];

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        window.set_background_color(125, 125, 125);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
            // REMOVE FOR DEBUGGING
            //if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            //    self.cpu.decode_execute();
            //}
            self.cpu.run_cycle();
        }
    }
}
