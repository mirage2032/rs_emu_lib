macro_rules! inc_rr {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<INC_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<INC_ $cdest>] {
                pub fn new() -> [<INC_ $cdest>] {
                    [<INC_ $cdest>] {
                        common: InstructionCommon::new(1, 6, true),
                    }
                }
            }

            impl Display for [<INC_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "INC {}", $cdest)
                }
            }

            impl BaseInstruction for [<INC_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<INC_ $cdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.[<$dest>] = cpu.registers.gp.[<$dest>].wrapping_add(1);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use inc_rr;
