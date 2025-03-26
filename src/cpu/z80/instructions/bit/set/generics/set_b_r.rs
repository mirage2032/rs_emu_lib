macro_rules! set_b_r {
        ($bit:literal, $src:expr,$srclit:literal,$opcode:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SET_ $bit _ $srclit>] {
                common: InstructionCommon,
            }

            impl [<SET_ $bit _ $srclit>] {
                pub fn new() -> [<SET_ $bit _ $srclit>] {
                    [<SET_ $bit _ $srclit>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<SET_ $bit _ $srclit>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SET {}, {}", $bit, $srclit)
                }
            }

            impl BaseInstruction for [<SET_ $bit _ $srclit>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SET_ $bit _ $srclit>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.$src |= (1 << $bit);
                    cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_SET_ $bit _ $srclit>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("cb",$opcode);

                test_instruction_parse!([<SET_ $bit _ $srclit>]);
            }
        }
    }
}

pub(crate) use set_b_r;
