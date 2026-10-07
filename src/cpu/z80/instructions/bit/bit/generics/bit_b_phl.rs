macro_rules! bit_b_phl {
        ($bit:literal, $opcode:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<BIT_ $bit _PHL>] {
                common: InstructionCommon,
            }

            impl [<BIT_ $bit _PHL>] {
                pub fn new() -> [<BIT_ $bit _PHL>] {
                    [<BIT_ $bit _PHL>] {
                        common: InstructionCommon::new(2, 12, true),
                    }
                }
            }

            impl Display for [<BIT_ $bit _PHL>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "BIT {}, (HL)", $bit)
                }
            }

            impl BaseInstruction for [<BIT_ $bit _PHL>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<BIT_ $bit _PHL>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let value = memory.read_8(cpu.registers.gp.hl)?;
                    alu::bit(&mut cpu.registers.gp.f, $bit, value, None);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use bit_b_phl;
