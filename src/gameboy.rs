use std::{rc::Rc, time};

use sdl2::{self, event::Event, keyboard::Keycode, Sdl};

use crate::{
    audio::Audio, bootrom::Bootrom, cartridge::Cartridge, cpu::Cpu, joypad::Button, lcd::LCD,
    peripherals::Peripherals,
};

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
        let mut peripherals = Peripherals::new(bootrom, cartridge);
        let audio = Audio::new(&sdl);
        peripherals.apu.set_callback(Rc::new(audio.0));
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
                    Event::KeyDown {
                        keycode: Some(k), ..
                    } => {
                        if k == Keycode::Escape {
                            break 'running;
                        }
                        key2joy(k).map(|j| {
                            self.peripherals
                                .joypad
                                .button_down(&mut self.cpu.interrupts, j);
                        });
                    }
                    Event::KeyUp {
                        keycode: Some(k), ..
                    } => {
                        key2joy(k).map(|j| self.peripherals.joypad.button_up(j));
                    }
                    _ => (),
                }
            }
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);
                self.peripherals
                    .timer
                    .emulate_cycle(&mut self.cpu.interrupts);
                self.peripherals.apu.emulate_cycle();
                if let Some(addr) = self.peripherals.ppu.oam_dma {
                    self.peripherals
                        .ppu
                        .oam_dma_emulate_cycle(self.peripherals.read(&self.cpu.interrupts, addr));
                }
                if self.peripherals.ppu.emulate_cycle(&mut self.cpu.interrupts) {
                    self.lcd.draw(self.peripherals.ppu.pixel_buffer());
                }

                elapsed += M_CYCLE_NANOS;
            }
        }
    }
}

fn key2joy(keycode: Keycode) -> Option<Button> {
    match keycode {
        Keycode::W => Some(Button::Up),
        Keycode::S => Some(Button::Down),
        Keycode::A => Some(Button::Left),
        Keycode::D => Some(Button::Right),
        Keycode::K => Some(Button::A),
        Keycode::J => Some(Button::B),
        Keycode::N => Some(Button::Select),
        Keycode::M => Some(Button::Start),
        _ => None,
    }
}
