macro_rules! cp_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<CP_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<CP_ $sdest>] {
                pub fn new() -> [<CP_ $sdest>] {
                    [<CP_ $sdest>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<CP_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "CP {}", $sdest)
                }
            }

            impl BaseInstruction for [<CP_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<CP_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    alu::cp8(&mut gp.f, gp.a, gp.$src);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use cp_r;
