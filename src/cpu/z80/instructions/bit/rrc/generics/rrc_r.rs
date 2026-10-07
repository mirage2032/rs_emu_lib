macro_rules! rrc_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RRC_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<RRC_ $sdest>] {
                pub fn new() -> [<RRC_ $sdest>] {
                    [<RRC_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<RRC_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "RRC {}", $sdest)
                }
            }

            impl BaseInstruction for [<RRC_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RRC_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.$src = alu::rrc8(&mut gp.f, gp.$src);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use rrc_r;
