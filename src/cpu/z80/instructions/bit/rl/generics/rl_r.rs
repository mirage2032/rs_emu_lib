macro_rules! rl_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RL_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<RL_ $sdest>] {
                pub fn new() -> [<RL_ $sdest>] {
                    [<RL_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<RL_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "RL {}", $sdest)
                }
            }

            impl BaseInstruction for [<RL_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RL_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    rl_r_setf!(gp.$src, gp.f);
                    cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_RL_ $sdest>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("cb",$opcode);

                test_instruction_parse!([<RL_ $sdest>]);
            }
        }
    }
}

pub(crate) use rl_r;
