use std::{env, fs::File, io::Read, process::exit, sync::mpsc::channel};

use bootrom::Bootrom;
use gameboy::GameBoy;

mod bootrom;
mod cpu;
mod decode;
mod fetch;
mod gameboy;
mod hram;
mod instructions;
mod lcd;
mod operands;
mod peripherals;
mod ppu;
mod registers;
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

    let (tx, rx) = channel::<bool>();
    ctrlc::set_handler(move || tx.send(true).unwrap()).unwrap();

    let bootrom_raw = file2vec(&args[1]);
    let bootrom = Bootrom::new(bootrom_raw.into());
    let mut gameboy = GameBoy::new(bootrom);
    gameboy.run(rx);
}
