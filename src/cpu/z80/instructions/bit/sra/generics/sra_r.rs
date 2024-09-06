macro_rules! sra_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SRA_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<SRA_ $sdest>] {
                pub fn new() -> [<SRA_ $sdest>] {
                    [<SRA_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<SRA_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SRA {}", $sdest)
                }
            }

            impl BaseInstruction for [<SRA_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SRA_ $sdest>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp[0];
                    sra_r_setf!(gp.$src, gp.f);
                    match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => {
                *val = val.wrapping_add(1) % 128;
            }
            _ => return Err("Invalid register".to_string()),
        }

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_SRA_ $sdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("cb",$opcode);

                test_instruction_parse!([<SRA_ $sdest>]);
            }
        }
    }
}

pub(crate) use sra_r;
