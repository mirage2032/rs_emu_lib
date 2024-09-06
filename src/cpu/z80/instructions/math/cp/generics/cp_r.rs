macro_rules! cp_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<CP_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<CP_ $sdest>] {
                pub fn new() -> [<CP_ $sdest>] {
                    [<CP_ $sdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<CP_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "CP {}", $sdest)
                }
            }

            impl BaseInstruction for [<CP_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<CP_ $sdest>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp[0];
                    cp_r_setf!(gp.a, gp.$src, gp.f);

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_CP_ $sdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<CP_ $sdest>]);
            }
        }
    }
}

pub(crate) use cp_r;
