// memory management unit

use std::path::Path;

use crate::{cartridge::Cartridge, serial::Serial, timer::Timer};

// NOTE: "word" in this context means 16-bit

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7FFF;
const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const SPRITE_OAM_START: u16 = 0xFE00;
const SPRITE_OAM_END: u16 = 0xFE9F;
const JOYPAD: u16 = 0xFF00;
const SERIAL_START: u16 = 0xFF01;
const SERIAL_END: u16 = 0xFF02;
const TIMER_START: u16 = 0xFF04;
const TIMER_END: u16 = 0xFF07;
const INTERRUPT_FLAG: u16 = 0xFF0F;
const SOUND_START: u16 = 0xFF10;
const SOUND_END: u16 = 0xFF26;
const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const INTERRUPT_ENABLE: u16 = 0xFFFF;

const WRAM_SIZE: u16 = 0x0FFF;
const VRAM_SIZE: u16 = 0x1FFF;
const HRAM_SIZE: u16 = 0x7E;

// can be read from or written to by the CPU
pub struct Bus {
    pub timer: Timer,
    rom: Cartridge,
    pub serial: Serial, // TODO: make private when done testing
    // internal ram
    working_ram: Vec<u8>,
    // stores graphic tiles
    // OAM stores data that tells the gameboy
    // which tiles to use to construct moving objects on the screen
    video_ram: Vec<u8>,
    high_ram: Vec<u8>,
}

impl Bus {
    pub fn new(rom_file: &Path) -> Self {
        let mut bus = Self {
            timer: Timer::new(),
            serial: Serial::new(),
            rom: Cartridge::new(),
            working_ram: vec![0; WRAM_SIZE as usize + 1],
            video_ram: vec![0; VRAM_SIZE as usize + 1],
            high_ram: vec![0; HRAM_SIZE as usize + 1],
        };

        bus.rom.load(rom_file).unwrap();
        println!("{}", bus.rom);

        // hardware registers
        bus.write_byte(0xFF00, 0xCF);
        bus.write_byte(0xFF01, 0x00);
        bus.write_byte(0xFF02, 0x7E);
        bus.write_byte(0xFF04, 0xAB);
        bus.write_byte(0xFF05, 0x00);
        bus.write_byte(0xFF06, 0x00);
        bus.write_byte(0xFF07, 0xF8);
        bus.write_byte(0xFF10, 0x80);
        bus.write_byte(0xFF11, 0xBF);
        bus.write_byte(0xFF12, 0xF3);
        bus.write_byte(0xFF14, 0xBF);
        bus.write_byte(0xFF16, 0x3F);
        bus.write_byte(0xFF17, 0x00);
        bus.write_byte(0xFF19, 0xBF);
        bus.write_byte(0xFF1A, 0x7F);
        bus.write_byte(0xFF1B, 0xFF);
        bus.write_byte(0xFF1C, 0x9F);
        bus.write_byte(0xFF1E, 0xFF);
        bus.write_byte(0xFF20, 0xFF);
        bus.write_byte(0xFF21, 0x00);
        bus.write_byte(0xFF22, 0x00);
        bus.write_byte(0xFF23, 0xBF);
        bus.write_byte(0xFF24, 0x77);
        bus.write_byte(0xFF25, 0xF3);
        bus.write_byte(0xFF26, 0xF1);
        bus.write_byte(0xFF40, 0x91);
        bus.write_byte(0xFF42, 0x00);
        bus.write_byte(0xFF43, 0x00);
        bus.write_byte(0xFF45, 0x00);
        bus.write_byte(0xFF47, 0xFC);
        bus.write_byte(0xFF48, 0xFF);
        bus.write_byte(0xFF49, 0xFF);
        bus.write_byte(0xFF4A, 0x00);
        bus.write_byte(0xFF4B, 0x00);

        bus
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // from cartridge, usually fixed bank
            ROM_START..=ROM_END => self.rom.read_byte(addr),
            VRAM_START..=VRAM_END => self.video_ram[addr as usize & VRAM_SIZE as usize],
            0xA000..=0xBFFF => self.rom.read_byte(addr),
            WRAM_START..=WRAM_END | 0xE000..=0xEFFF | 0xF000..=0xFDFF => {
                self.working_ram[addr as usize & WRAM_SIZE as usize]
            }
            // sprite attribute table
            SPRITE_OAM_START..=SPRITE_OAM_END => {
                self.video_ram[addr as usize - SPRITE_OAM_START as usize]
            }
            // prohibited area
            0xFEA0..=0xFEFF => 0,
            // I/O registers
            JOYPAD => 0, // TODO: implement joypad input
            SERIAL_START..=SERIAL_END => self.serial.read_byte(addr),
            TIMER_START..=TIMER_END => self.timer.read_byte(addr),
            INTERRUPT_FLAG => 0,
            SOUND_START..=SOUND_END => 0,
            // high ram (HRAM)
            HRAM_START..=HRAM_END => self.high_ram[addr as usize & HRAM_SIZE as usize],
            INTERRUPT_ENABLE => 0,

            _ => 0,
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            // from cartridge, usually fixed bank
            ROM_START..=ROM_END => self.rom.write_byte(addr, value),
            VRAM_START..=VRAM_END => self.video_ram[addr as usize & VRAM_SIZE as usize] = value,
            0xA000..=0xBFFF => self.rom.write_byte(addr, value),
            WRAM_START..=WRAM_END => self.working_ram[addr as usize & WRAM_SIZE as usize] = value,
            // sprite attribute table
            SPRITE_OAM_START..=SPRITE_OAM_END => {
                self.video_ram[addr as usize - SPRITE_OAM_START as usize] = value
            }
            // prohibited area
            0xFEA0..=0xFEFF => {}
            // I/O registers
            JOYPAD => {}
            SERIAL_START..=SERIAL_END => self.serial.write_byte(addr, value),
            TIMER_START..=TIMER_END => self.timer.write_byte(addr, value),
            INTERRUPT_FLAG => {}
            SOUND_START..=SOUND_END => {}
            // high ram (HRAM)
            HRAM_START..=HRAM_END => self.high_ram[addr as usize & HRAM_SIZE as usize] = value,
            // interrupt enable register (IE)
            INTERRUPT_ENABLE => {}
            _ => {}
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr + 1, (value >> 8) as u8);
    }
}
