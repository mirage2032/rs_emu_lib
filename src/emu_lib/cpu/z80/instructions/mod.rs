use crate::emu_lib::cpu::ExecutableInstruction;
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::Memory;

mod nop;
mod ld;

pub fn decode(memory: &Memory, pos: u16) -> Box<dyn ExecutableInstruction<Z80>> {
    let instruction: Box<dyn ExecutableInstruction<Z80>> = match memory.read8(pos) {
        0x00 => Box::new(nop::NOP::new()),
        0x01 => Box::new(ld::ld_bc_nn::LD_BC_NN::new(memory, pos)),
        _ => unimplemented!()
    };
    instruction
}