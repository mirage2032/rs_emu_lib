macro_rules! dec_rr {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<DEC_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<DEC_ $cdest>] {
                pub fn new() -> [<DEC_ $cdest>] {
                    [<DEC_ $cdest>] {
                        common: InstructionCommon::new(1, 6, true),
                    }
                }
            }

            impl Display for [<DEC_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "DEC {}", $cdest)
                }
            }

            impl BaseInstruction for [<DEC_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<DEC_ $cdest>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = cpu.registers.gp[0].[<$dest>].wrapping_sub(1);
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_DEC_ $cdest>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<DEC_ $cdest>]);
            }
        }
    }
}

pub(crate) use dec_rr;
