macro_rules! set_b_pixd {
        ($bit:literal,$opcode:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<SET_ $bit _PIXD>] {
                common: InstructionCommon,
                d: i8,
            }

            impl [<SET_ $bit _PIXD>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<SET_ $bit _PIXD>], MemoryReadError> {
                    Ok([<SET_ $bit _PIXD>] {
                        common: InstructionCommon::new(4, 23, true),
                        d: memory.read_8(pos.wrapping_add(2))? as i8,
                    })}
                pub fn new_with_value(d: u8) -> [<SET_ $bit _PIXD>] {
                    [<SET_ $bit _PIXD>] {
                        common: InstructionCommon::new(4, 23, true),
                        d: d as i8,
                        }
                }
            }

            impl Display for [<SET_ $bit _PIXD>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "SET {}, (IX+0x{:02X})",$bit , self.d)
                }
            }

            impl BaseInstruction for [<SET_ $bit _PIXD>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xdd,0xcb,self.d as u8,hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<SET_ $bit _PIXD>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let offset = cpu.registers.ix.wrapping_add(self.d as u16);
                    let mut value = memory.read_8(offset as u16)?;
                    value = value | (1 << $bit);
                    memory.write_8(offset as u16, value)?;
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use set_b_pixd;
