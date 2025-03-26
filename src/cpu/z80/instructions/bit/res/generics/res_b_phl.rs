macro_rules! res_b_phl {
        ($bit:literal, $opcode:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<RES_ $bit _PHL>] {
                common: InstructionCommon,
            }

            impl [<RES_ $bit _PHL>] {
                pub fn new() -> [<RES_ $bit _PHL>] {
                    [<RES_ $bit _PHL>] {
                        common: InstructionCommon::new(2, 15, true),
                    }
                }
            }

            impl Display for [<RES_ $bit _PHL>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "BIT {}, (HL)", $bit)
                }
            }

            impl BaseInstruction for [<RES_ $bit _PHL>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xcb,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<RES_ $bit _PHL>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let mut value = memory.read_8(cpu.registers.gp.hl)?;
                    //set bit to 0
                    value = value & !(1 << $bit);
                    memory.write_8(cpu.registers.gp.hl, value)?;
                    cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;

                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_RES_ $bit _PHL>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("cb",$opcode);

                test_instruction_parse!([<RES_ $bit _PHL>]);
            }
        }
    }
}

pub(crate) use res_b_phl;
