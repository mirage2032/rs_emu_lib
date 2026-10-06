macro_rules! add_r_r {
        ($dest:expr,$src:expr,$opcode:literal,$cdest:literal,$csrc:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<ADD_ $cdest _ $csrc>] {
                common: InstructionCommon,
            }

            impl [<ADD_ $cdest _ $csrc>] {
                pub fn new() -> [<ADD_ $cdest _ $csrc>] {
                    [<ADD_ $cdest _ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<ADD_ $cdest _ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "ADD {}, {}", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<ADD_ $cdest _ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<ADD_ $cdest _ $csrc>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.[<$dest>] = alu::add8(&mut gp.f, gp.[<$dest>], gp.[<$src>], false);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use add_r_r;
