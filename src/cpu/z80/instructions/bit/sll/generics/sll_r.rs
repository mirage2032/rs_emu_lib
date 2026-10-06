macro_rules! sll_r {
        ($src:expr,$opcode:literal,$sdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SLL_ $sdest>] {
                common: InstructionCommon,
            }

            impl [<SLL_ $sdest>] {
                pub fn new() -> [<SLL_ $sdest>] {
                    [<SLL_ $sdest>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<SLL_ $sdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SLL {}", $sdest)
                }
            }

            impl BaseInstruction for [<SLL_ $sdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SLL_ $sdest>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    gp.$src = alu::sll8(&mut gp.f, gp.$src);

                    Ok(())
                }
            }
        }
    }
}

pub(crate) use sll_r;
