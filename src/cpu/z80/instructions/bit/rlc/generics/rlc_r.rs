macro_rules! rlc_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RLC_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<RLC_ $sdest>] {
                pub fn new() -> [<RLC_ $sdest>] {
                    [<RLC_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<RLC_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "RLC {}", $sdest)
                }
            }

            impl BaseInstruction for [<RLC_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RLC_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.$src = alu::rlc8(&mut gp.f, gp.$src);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use rlc_r;
