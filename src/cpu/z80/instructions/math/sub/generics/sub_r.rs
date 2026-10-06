macro_rules! sub_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SUB_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<SUB_ $sdest>] {
                pub fn new() -> [<SUB_ $sdest>] {
                    [<SUB_ $sdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<SUB_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SUB {}", $sdest)
                }
            }

            impl BaseInstruction for [<SUB_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SUB_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.a = alu::sub8(&mut gp.f, gp.a, gp.$src, false);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use sub_r;
