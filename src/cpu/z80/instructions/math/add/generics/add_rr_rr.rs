macro_rules! add_rr_rr {
        ($dest:expr,$src:expr,$opcode:literal,$cdest:literal,$csrc:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<ADD_ $cdest _ $csrc>] {
                common: InstructionCommon,
            }

            impl [<ADD_ $cdest _ $csrc>] {
                pub fn new() -> [<ADD_ $cdest _ $csrc>] {
                    [<ADD_ $cdest _ $csrc>] {
                        common: InstructionCommon::new(1, 11, true),
                    }
                }
            }

            impl Display for [<ADD_ $cdest _ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "ADD {}, {}", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<ADD_ $cdest _ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<ADD_ $cdest _ $csrc>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    add_rr_rr_setf!(
                        cpu.registers.gp.[<$dest>],
                        cpu.registers.gp.[<$src>],
                        cpu.registers.gp.f
                    );
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_ADD_ $cdest _ $csrc>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<ADD_ $cdest _ $csrc>]);
            }
        }
    }
}

pub(crate) use add_rr_rr;
