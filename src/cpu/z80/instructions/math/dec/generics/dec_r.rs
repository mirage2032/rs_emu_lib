macro_rules! dec_r {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<DEC_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<DEC_ $cdest>] {
                pub fn new() -> [<DEC_ $cdest>] {
                    [<DEC_ $cdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<DEC_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "DEC {}", $cdest)
                }
            }

            impl BaseInstruction for [<DEC_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<DEC_ $cdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.[<$dest>] = alu::dec8(&mut gp.f, gp.[<$dest>]);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use dec_r;
