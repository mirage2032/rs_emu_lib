macro_rules! ld_pnn_rr {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_PNN_ $cdest>] {
                common: InstructionCommon,
                nn: u16,
            }

            impl [<LD_PNN_ $cdest>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_PNN_ $cdest>],String> {
                    Ok([<LD_PNN_ $cdest>] {
                        common: InstructionCommon::new(3, 16, true),
                        nn:memory.read_16(pos.wrapping_add(1))?,
                    })
                }

                pub fn new_with_value(nn: u16) -> [<LD_PNN_ $cdest>] {
                    [<LD_PNN_ $cdest>] {
                        common: InstructionCommon::new(3, 16, true),
                        nn,
                    }
                }
            }

            impl Display for [<LD_PNN_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD ({:04}), {}", self.nn, $cdest)
                }
            }

            impl BaseInstruction for [<LD_PNN_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    let nn_lsb = self.nn.to_le_bytes();
                    vec![hex!( $opcode )[0], nn_lsb[0], nn_lsb[1]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_PNN_ $cdest>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    memory.write_16(self.nn, cpu.registers.gp[0].[<$dest>])?;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_PNN_ $cdest>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_PNN_ $cdest>],[0x1234]);
            }
        }
    }
}

pub(crate) use ld_pnn_rr;
