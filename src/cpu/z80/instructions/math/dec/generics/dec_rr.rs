macro_rules! dec_rr {
        ($dest:expr,$opcode:literal,$cdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<DEC_ $cdest>] {
                common: InstructionCommon,
            }

            impl [<DEC_ $cdest>] {
                pub fn new() -> [<DEC_ $cdest>] {
                    [<DEC_ $cdest>] {
                        common: InstructionCommon::new(1, 6, true),
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
                    cpu.registers.gp.[<$dest>] = cpu.registers.gp.[<$dest>].wrapping_sub(1);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use dec_rr;
