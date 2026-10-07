macro_rules! res_b_r {
        ($bit:literal, $src:expr,$srclit:literal,$opcode:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RES_ $bit _ $srclit>] {
                common: InstructionCommon,
            }

            impl [<RES_ $bit _ $srclit>] {
                pub fn new() -> [<RES_ $bit _ $srclit>] {
                    [<RES_ $bit _ $srclit>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<RES_ $bit _ $srclit>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "RES {}, {}", $bit, $srclit)
                }
            }

            impl BaseInstruction for [<RES_ $bit _ $srclit>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RES_ $bit _ $srclit>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.$src &= !(1 << $bit);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use res_b_r;
