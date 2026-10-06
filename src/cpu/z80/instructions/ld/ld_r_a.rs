use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct LD_R_A {
    common: InstructionCommon,
}

impl LD_R_A {
    pub fn new() -> LD_R_A {
        LD_R_A {
            common: InstructionCommon::new(2, 9, true),
        }
    }
}

impl Display for LD_R_A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD R, A",)
    }
}

impl BaseInstruction for LD_R_A {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xed,0x4f]
    }
}

impl ExecutableInstruction<Z80> for LD_R_A {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
        cpu.registers.r = cpu.registers.gp.a;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bit_7_of_r_survives_until_ld_a_r_reads_it() {
        use crate::cpu::z80::Z80;
        use crate::emulator::Emulator;
        use crate::memory::Memory;

        // LD A,0xFF; LD R,A; NOP; LD A,R
        let mut memory = Memory::new_full_ram();
        memory.load(&[0x3E, 0xFF, 0xED, 0x4F, 0x00, 0xED, 0x5F], true).unwrap();
        let mut emu: Emulator<Z80> = Emulator::new_w_mem(memory);
        for _ in 0..4 {
            emu.step().unwrap();
        }
        // R is 0xFF after LD R,A; the NOP's fetch makes it 0x80, LD A,R's two 0x82.
        assert_eq!((emu.cpu.registers.gp.a, emu.cpu.registers.r), (0x82, 0x82));
    }
}
