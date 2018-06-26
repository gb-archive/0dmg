use super::GameBoy;

use super::audio::AudioController;
use super::video::VideoController;

pub struct MemoryData {
    wram: [u8; 0x2000],
    stack_ram: [u8; 0x80],
    boot_rom: [u8; 0x100],
    game_rom: Vec<u8>,
    boot_rom_mapped: bool,
}

impl MemoryData {
    pub fn new(game_rom: Vec<u8>) -> Self {
        Self {
            wram: [0x00; 0x2000],
            stack_ram: [0x00; 0x80],
            game_rom,
            boot_rom_mapped: true,
            boot_rom: BOOT_ROM.clone(),
        }
    }
}

pub trait MemoryController {
    fn mem(&self, addr: u16) -> u8;
    fn set_mem(&mut self, addr: u16, value: u8);
}

impl MemoryController for GameBoy {
    fn mem(&self, addr: u16) -> u8 {
        let value = if self.mem.boot_rom_mapped && addr <= 0x00FF {
            // boot ROM, until unmapped to expose initial bytes of game ROM
            self.mem.boot_rom[addr as usize]
        } else if addr <= 0x7FFF {
            // first page of game ROM
            let value = self.mem.game_rom[addr as usize];
            // println!("    ; game_rom[${:02x}] == ${:02x}", addr, value);
            value
        } else if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.vram(i)
        } else if 0xC000 <= addr && addr <= 0xDFFF {
            let i: usize = (addr - 0xC000) as usize;
            self.mem.wram[i]
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            self.mem.stack_ram[i]
        } else if 0xFF10 <= addr && addr <= 0xFF26 {
            let i = (addr - 0xFF10) as usize;
            self.audio_register(i)
        } else if addr == 0xFF40 {
            self.lcdc()
        } else if addr == 0xFF42 {
            self.scy()
        } else if addr == 0xFF43 {
            self.scx()
        } else if addr == 0xFF44 {
            self.ly()
        } else if addr == 0xFF47 {
            self.bgp()
        } else if addr == 0xFF50 {
            if self.mem.boot_rom_mapped {
                0x01
            } else {
                0x00
            }
        } else {
            panic!("I don't know how to get memory address ${:04x}.", addr);
        };

        value
    }

    fn set_mem(&mut self, addr: u16, value: u8) {
        if 0x8000 <= addr && addr <= 0x9FFF {
            let i: usize = (addr - 0x8000) as usize;
            self.set_vram(i, value);
        } else if 0xC000 <= addr && addr <= 0xDFFF {
            let i: usize = (addr - 0xC000) as usize;
            self.mem.wram[i] = value;
        } else if 0xFF80 <= addr && addr <= 0xFFFE {
            let i: usize = (addr - 0xFF80) as usize;
            self.mem.stack_ram[i] = value;
        } else if 0xFF10 <= addr && addr <= 0xFF26 {
            let i = (addr - 0xFF10) as usize;
            self.set_audio_register(i, value);
        } else if addr == 0xFF40 {
            self.set_lcdc(value);
        } else if addr == 0xFF42 {
            self.set_scy(value);
        } else if addr == 0xFF43 {
            self.set_scx(value);
        } else if addr == 0xFF44 {
            self.set_ly(value);
        } else if addr == 0xFF47 {
            self.set_bgp(value);
        } else if addr == 0xFF50 {
            if value != 0x01 {
                panic!(
                    "got unexpected value (not 0x01) written to 0xFF50 boot rom disable register"
                );
            }
            self.mem.boot_rom_mapped = false;
        } else {
            panic!(
                "I don't know how to set memory address ${:04x} (to ${:02x}).",
                addr, value
            );
        }
    }
}

const BOOT_ROM: &'static [u8; 0x0100] = &[
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];
