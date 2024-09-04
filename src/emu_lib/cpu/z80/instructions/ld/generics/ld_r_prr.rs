macro_rules! ld_r_prr {
    ($dest:ident,$src:ident,$opcode:literal,$cdest:literal,$csrc:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _P $csrc>] {
                common: InstructionCommon,
            }

            impl [<LD_ $cdest _P $csrc>] {
                pub fn new() -> [<LD_ $cdest _P $csrc>] {
                    [<LD_ $cdest _P $csrc>] {
                        common: InstructionCommon::new(1, 7, true),
                    }
                }
            }

            impl Display for [<LD_ $cdest _P $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, ({})", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _P $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _P $csrc>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = memory.read_8(cpu.registers.gp[0].[<$src>])?;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_ $cdest _P $csrc>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_ $cdest _P $csrc>]);
            }
        }
    }
}

pub(crate) use ld_r_prr;
