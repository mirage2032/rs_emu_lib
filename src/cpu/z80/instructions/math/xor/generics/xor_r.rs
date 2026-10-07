macro_rules! xor_r {
        ($src:expr,$opcode:literal,$csrc:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<XOR_ $csrc>] {
                common: InstructionCommon,
            }

            impl [<XOR_ $csrc>] {
                pub fn new() -> [<XOR_ $csrc>] {
                    [<XOR_ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<XOR_ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "XOR {}", $csrc)
                }
            }

            impl BaseInstruction for [<XOR_ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<XOR_ $csrc>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.a = alu::xor8(&mut gp.f, gp.a, gp.[<$src>]);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use xor_r;
