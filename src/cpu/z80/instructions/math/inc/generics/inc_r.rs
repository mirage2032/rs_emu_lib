macro_rules! inc_r {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<INC_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<INC_ $cdest>] {
                pub fn new() -> [<INC_ $cdest>] {
                    [<INC_ $cdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<INC_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "INC {}", $cdest)
                }
            }

            impl BaseInstruction for [<INC_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<INC_ $cdest>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp[0];
                    inc_r_setf!(&mut gp.[<$dest>], &mut gp.f);
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_INC_ $cdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<INC_ $cdest>]);
            }
        }
    }
}

pub(crate) use inc_r;
