macro_rules! bit_b_r {
        ($bit:literal, $src:expr,$srclit:literal,$opcode:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<BIT_ $bit _ $srclit>] {
                common: InstructionCommon,
            }

            impl [<BIT_ $bit _ $srclit>] {
                pub fn new() -> [<BIT_ $bit _ $srclit>] {
                    [<BIT_ $bit _ $srclit>] {
                        common: InstructionCommon::new(2, 8, true),
                    }
                }
            }

            impl Display for [<BIT_ $bit _ $srclit>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "BIT {}, {}", $bit, $srclit)
                }
            }

            impl BaseInstruction for [<BIT_ $bit _ $srclit>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<BIT_ $bit _ $srclit>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    bit_b_r_setf!(gp.$src,$bit, gp.f);
                    cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_BIT_ $bit _ $srclit>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("cb",$opcode);

                test_instruction_parse!([<BIT_ $bit _ $srclit>]);
            }
        }
    }
}

pub(crate) use bit_b_r;
