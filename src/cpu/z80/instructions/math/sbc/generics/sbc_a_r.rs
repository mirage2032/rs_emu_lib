macro_rules! sbc_a_r {
        ($src:expr,$opcode:literal,$csrc:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SBC_A_ $csrc>] {
                common: InstructionCommon,
            }

            impl [<SBC_A_ $csrc>] {
                pub fn new() -> [<SBC_A_ $csrc>] {
                    [<SBC_A_ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<SBC_A_ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SBC A, {}", $csrc)
                }
            }

            impl BaseInstruction for [<SBC_A_ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SBC_A_ $csrc>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    sbc_r_r!(cpu.registers.gp.a,
                        cpu.registers.gp.[<$src>],
                        cpu.registers.gp.f
                    );
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_SBC_A_ $csrc>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<SBC_A_ $csrc>]);
            }
        }
    }
}

pub(crate) use sbc_a_r;
