macro_rules! sla_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SLA_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<SLA_ $sdest>] {
                pub fn new() -> [<SLA_ $sdest>] {
                    [<SLA_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<SLA_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SLA {}", $sdest)
                }
            }

            impl BaseInstruction for [<SLA_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SLA_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.$src = alu::sla8(&mut gp.f, gp.$src);
                    cpu.registers.inc_r();

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use sla_r;
