macro_rules! rst_n {
        ($dst:expr,$opcode:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RST_ $dst>] {
                common: InstructionCommon,
            }

            impl [<RST_ $dst>] {
                pub fn new() -> [<RST_ $dst>] {
                    [<RST_ $dst>] {
                        common: InstructionCommon::new(1, 11, false),
                    }
                }
            }

            impl Display for [<RST_ $dst>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "RST {}", $dst)
                }
            }

            impl BaseInstruction for [<RST_ $dst>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RST_ $dst>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    push_16!(cpu.registers.pc.wrapping_add(1), memory, cpu.registers.sp);
                    cpu.registers.pc = [<$dst>];
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use rst_n;
