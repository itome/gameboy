use std::{env, fs::File, io::Read, process::exit};

use bootrom::Bootrom;
use cartridge::Cartridge;
use gameboy::GameBoy;

mod apu;
mod audio;
mod bootrom;
mod cartridge;
mod cpu;
mod decode;
mod fetch;
mod gameboy;
mod hram;
mod instructions;
mod interrupts;
mod joypad;
mod lcd;
mod mbc;
mod operands;
mod peripherals;
mod ppu;
mod registers;
mod timer;
mod wram;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

fn file2vec(fname: &String) -> Vec<u8> {
    if let Ok(mut file) = File::open(fname) {
        let mut ret = vec![];
        file.read_to_end(&mut ret).unwrap();
        ret
    } else {
        panic!("Cannot open {}.", fname);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("The file name argument is required.");
        exit(1);
    }

    let bootrom_raw = file2vec(&args[1]);
    let cartridge_raw = file2vec(&args[2]);
    let bootrom = Bootrom::new(bootrom_raw.into());
    let cartridge = Cartridge::new(cartridge_raw.into());
    let mut gameboy = GameBoy::new(bootrom, cartridge);
    gameboy.run();
}
