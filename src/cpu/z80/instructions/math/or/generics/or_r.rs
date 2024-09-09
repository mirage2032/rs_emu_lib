macro_rules! or_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<OR_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<OR_ $sdest>] {
                pub fn new() -> [<OR_ $sdest>] {
                    [<OR_ $sdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<OR_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "OR {}", $sdest)
                }
            }

            impl BaseInstruction for [<OR_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<OR_ $sdest>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    or_r_setf!(gp.a, gp.$src, gp.f);

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_OR_ $sdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<OR_ $sdest>]);
            }
        }
    }
}

pub(crate) use or_r;
