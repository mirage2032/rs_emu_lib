macro_rules! pop_rr {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<POP_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<POP_ $cdest>] {
                pub fn new() -> [<POP_ $cdest>] {
                    [<POP_ $cdest>] {
                        common: InstructionCommon::new(1, 10, true),
                    }
                }
            }

            impl Display for [<POP_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "POP {}", $cdest)
                }
            }

            impl BaseInstruction for [<POP_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<POP_ $cdest>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = pop_16!(memory, cpu.registers.sp);
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_POP_ $cdest>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<POP_ $cdest>]);
            }
        }
    }
}

pub(crate) use pop_rr;
