macro_rules! push_rr {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<PUSH_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<PUSH_ $cdest>] {
                pub fn new() -> [<PUSH_ $cdest>] {
                    [<PUSH_ $cdest>] {
                        common: InstructionCommon::new(1, 11, true),
                    }
                }
            }

            impl Display for [<PUSH_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "PUSH {}", $cdest)
                }
            }

            impl BaseInstruction for [<PUSH_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<PUSH_ $cdest>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    push_16!(cpu.registers.gp[0].[<$dest>],memory, cpu.registers.sp);
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_PUSH_ $cdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<PUSH_ $cdest>]);
            }
        }
    }
}

pub(crate) use push_rr;
