macro_rules! srl_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SRL_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<SRL_ $sdest>] {
                pub fn new() -> [<SRL_ $sdest>] {
                    [<SRL_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<SRL_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SRL {}", $sdest)
                }
            }

            impl BaseInstruction for [<SRL_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SRL_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.$src = alu::srl8(&mut gp.f, gp.$src);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use srl_r;
