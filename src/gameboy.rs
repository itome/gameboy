use std::time;

use sdl2::{self, event::Event, Sdl};

use crate::{bootrom::Bootrom, cartridge::Cartridge, cpu::Cpu, lcd::LCD, peripherals::Peripherals};

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: LCD,
    sdl: Sdl,
}

impl GameBoy {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = LCD::new(&sdl, 4);
        let peripherals = Peripherals::new(bootrom, cartridge);
        let cpu = Cpu::new();
        Self {
            cpu,
            peripherals,
            lcd,
            sdl,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let time = time::Instant::now();
        let mut elapsed = 0;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => (),
                }
            }
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);
                self.peripherals
                    .timer
                    .emulate_cycle(&mut self.cpu.interrupts);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(self.peripherals.ppu.pixel_buffer());
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}
