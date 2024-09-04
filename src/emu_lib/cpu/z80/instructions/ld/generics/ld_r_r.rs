macro_rules! ld_r_r {
        ($dest:expr,$src:expr,$opcode:literal,$cdest:literal,$csrc:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _ $csrc>] {
                common: InstructionCommon,
            }

            impl [<LD_ $cdest _ $csrc>] {
                pub fn new() -> [<LD_ $cdest _ $csrc>] {
                    [<LD_ $cdest _ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<LD_ $cdest _ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, {}", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _ $csrc>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = cpu.registers.gp[0].[<$src>];
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_ $cdest _ $csrc>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_ $cdest _ $csrc>]);
            }
        }
    }
}

pub(crate) use ld_r_r;
