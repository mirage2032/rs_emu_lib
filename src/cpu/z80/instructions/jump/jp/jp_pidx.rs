use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::cpu::z80::{IndexRegister, IX, IY};
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct JP_PIDX<R: IndexRegister> {
    common: InstructionCommon,
    index: PhantomData<R>,
}

pub type JP_PIX = JP_PIDX<IX>;
pub type JP_PIY = JP_PIDX<IY>;

impl<R: IndexRegister> JP_PIDX<R> {
    pub fn new() -> Self {
        Self {
            common: InstructionCommon::new(2, 8, false),
            index: PhantomData,
        }
    }
}

impl<R: IndexRegister> Display for JP_PIDX<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP ({name})", name = R::NAME)
    }
}

impl<R: IndexRegister> BaseInstruction for JP_PIDX<R> {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![R::PREFIX, 0xE9]
    }
}

impl<R: IndexRegister> ExecutableInstruction<Z80> for JP_PIDX<R> {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = R::get(&cpu.registers);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn r_counts_up_in_its_low_7_bits() {
        use crate::cpu::z80::Z80;
        use crate::emulator::Emulator;
        use crate::memory::Memory;

        let mut memory = Memory::new_full_ram();
        memory.load(&[0xDD, 0xE9], true).unwrap(); // two opcode fetches
        let mut emu: Emulator<Z80> = Emulator::new_w_mem(memory);
        emu.cpu.registers.r = 0xFE;
        emu.step().unwrap();
        assert_eq!(emu.cpu.registers.r, 0x80); // 0xFE -> 0xFF -> 0x80
    }
}
