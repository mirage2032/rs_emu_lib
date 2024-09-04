macro_rules! ld_prr_r {
    ($dest:ident,$src:ident,$opcode:literal,$sdest:literal,$ssrc:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_P $sdest _ $ssrc>] {
                common: InstructionCommon,
            }

            impl [<LD_P $sdest _ $ssrc>] {
                pub fn new() -> [<LD_P $sdest _ $ssrc>] {
                    [<LD_P $sdest _ $ssrc>] {
                        common: InstructionCommon::new(1, 7, true),
                    }
                }
            }

            impl Display for [<LD_P $sdest _ $ssrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD ({}), {}", $sdest, $ssrc)
                }
            }

            impl BaseInstruction for [<LD_P $sdest _ $ssrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_P $sdest _ $ssrc>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    memory.write_8(cpu.registers.gp[0].[<$dest>], cpu.registers.gp[0].[<$src>])?;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_P $sdest _ $ssrc>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_P $sdest _ $ssrc>]);
            }
        }
    }
}

pub(crate) use ld_prr_r;
