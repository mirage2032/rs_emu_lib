use crate::emu_lib::cpu::{ExecutableInstruction, InstructionDecoder};
use crate::emu_lib::cpu::z80::instructions::{ex, halt, ld, math, nop, rlca, rrca};
use crate::emu_lib::cpu::z80::Z80;
use crate::emu_lib::memory::ReadableMemory;

impl InstructionDecoder for Z80 {
    fn decode(memory: &impl ReadableMemory, pos: u16) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String> {
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match memory.read_8(pos)? {
            0x00u8 => Box::new(nop::NOP::new()),
            0x01 => Box::new(ld::ld_bc_nn::LD_BC_NN::new(memory, pos)?),
            0x02 => Box::new(ld::ld_pbc_a::LD_PBC_A::new()),
            0x03 => Box::new(math::inc::inc_bc::INC_BC::new()),
            0x04 => Box::new(math::inc::inc_b::INC_B::new()),
            0x05 => Box::new(math::dec::dec_b::DEC_B::new()),
            0x06 => Box::new(ld::ld_b_n::LD_B_N::new(memory, pos)?),
            0x07 => Box::new(rlca::RLCA::new()),
            0x08 => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
            0x09 => Box::new(math::add::add_hl_bc::ADD_HL_BC::new()),
            0x0A => Box::new(ld::ld_a_pbc::LD_A_PBC::new()),
            0x0B => Box::new(math::dec::dec_bc::DEC_BC::new()),
            0x0C => Box::new(math::inc::inc_c::INC_C::new()),
            0x0D => Box::new(math::dec::dec_c::DEC_C::new()),
            0x0E => Box::new(ld::ld_c_n::LD_C_N::new(memory, pos)?),
            0x0F => Box::new(rrca::RRCA::new()),
            0x76 => Box::new(halt::Halt::new()),
            _ => unimplemented!()
        };
        Ok(instruction)
    }
}