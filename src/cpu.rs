use crate::{
    instructions::{go, step},
    interrupts::{Interrupts, JOYPAD, SERIAL, STAT, TIMER, VBLANK},
    peripherals::Peripherals,
    registers::Registers,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[derive(Default)]
pub struct Ctx {
    pub opcode: u8,
    pub cb: bool,
    pub int: bool,
}
pub struct Cpu {
    pub regs: Registers,
    pub ctx: Ctx,
    pub interrupts: Interrupts,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
            interrupts: Interrupts::default(),
        }
    }

    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        if self.ctx.int {
            self.call_isr(bus);
        } else {
            self.decode(bus);
        }
    }

    fn call_isr(&mut self, bus: &mut Peripherals) {
        step!((), {
            0: if let Some(v) = self.push16(bus, self.regs.pc) {
                let highest_int: u8 = 1 << self.interrupts.get_interrupt().trailing_zeros();
                self.interrupts.intr_flags &= !highest_int;
                self.regs.pc = match highest_int {
                    VBLANK => 0x0040,
                    STAT => 0x0048,
                    TIMER => 0x0050,
                    SERIAL => 0x0058,
                    JOYPAD => 0x0060,
                    _ => panic!("Invalid interrupt: {:02x}", highest_int),
                };
                return go!(1);
            },
            1: {
                self.interrupts.ime = false;
                go!(0);
                self.fetch(bus);
            },
        });
    }
}
