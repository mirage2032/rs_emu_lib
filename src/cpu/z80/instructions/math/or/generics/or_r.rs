macro_rules! or_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<OR_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<OR_ $sdest>] {
                pub fn new() -> [<OR_ $sdest>] {
                    [<OR_ $sdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<OR_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "OR {}", $sdest)
                }
            }

            impl BaseInstruction for [<OR_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<OR_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.a = alu::or8(&mut gp.f, gp.a, gp.$src);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use or_r;
