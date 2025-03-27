macro_rules! bit_b_piyd {
        ($bit:literal,$opcode:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<BIT_ $bit _PIYD>] {
                common: InstructionCommon,
                d: i8,
            }

            impl [<BIT_ $bit _PIYD>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<BIT_ $bit _PIYD>], MemoryReadError> {
                    Ok([<BIT_ $bit _PIYD>] {
                        common: InstructionCommon::new(4, 20, true),
                        d: memory.read_8(pos.wrapping_add(2))? as i8,
                    })}
                pub fn new_with_value(d: u8) -> [<BIT_ $bit _PIYD>] {
                    [<BIT_ $bit _PIYD>] {
                        common: InstructionCommon::new(4, 20, true),
                        d: d as i8,
                        }
                }
            }

            impl Display for [<BIT_ $bit _PIYD>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "BIT {}, (IY+0x{:02X})",$bit , self.d)
                }
            }

            impl BaseInstruction for [<BIT_ $bit _PIYD>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xfd,0xcb,self.d as u8,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<BIT_ $bit _PIYD>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let offset = cpu.registers.iy.wrapping_add(self.d as u16);
                    let value = memory.read_8(offset as u16)?;
                    bit_b_r_setf!(value,$bit, cpu.registers.gp.f);
                    cpu.registers.r = cpu.registers.r.wrapping_add(1) % 128;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_BIT_ $bit _PIYD>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!("fd cb __", $opcode);

                test_instruction_parse!([<BIT_ $bit _PIYD>],[0x44]);
            }
        }
    }
}

pub(crate) use bit_b_piyd;
