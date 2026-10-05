use std::fmt;
use std::fmt::Display;

use crate::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionCommon};
use crate::cpu::z80::Z80;
use crate::io::IO;
use crate::memory::Memory;

#[derive(Debug)]
pub struct JP_PIX {
    common: InstructionCommon,
}

impl JP_PIX {
    pub fn new() -> JP_PIX {
        JP_PIX {
            common: InstructionCommon::new(2, 8, false),
        }
    }
}

impl Display for JP_PIX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JP (IX)")
    }
}

impl BaseInstruction for JP_PIX {
    fn common(&self) -> &InstructionCommon {
        &self.common
    }
    fn to_bytes(&self) -> Vec<u8> {
        vec![0xDD, 0xE9]
    }
}

impl ExecutableInstruction<Z80> for JP_PIX {
    fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _io: &mut IO) -> Result<(), String> {
        cpu.registers.pc = cpu.registers.ix;
        cpu.registers.inc_r();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::test::*;
    use crate::cpu::z80::test::*;

    test_z80!("dd", "e9");
    test_instruction_parse!(JP_PIX);

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
