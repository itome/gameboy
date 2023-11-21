use crate::{peripherals::Peripherals, registers::Registers};

#[derive(Default)]
pub struct Ctx {
    pub opcode: u8,
    pub cb: bool,
}
pub struct Cpu {
    pub regs: Registers,
    pub ctx: Ctx,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}
