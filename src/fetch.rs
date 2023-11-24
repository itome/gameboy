use crate::{cpu::Cpu, peripherals::Peripherals};

impl Cpu {
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        if self.interrupts.ime && self.interrupts.get_interrupts() > 0 {
            self.ctx.int = true;
        } else {
            self.ctx.int = false;
        }
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }
}
