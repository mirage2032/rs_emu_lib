macro_rules! ld_misc_rr_pnn {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::item! {
            #[derive(Debug)]
            pub struct [<LD_MISC_ $cdest _PNN>] {
                common: InstructionCommon,
                nn: u16,
            }

            impl [<LD_MISC_ $cdest _PNN>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_MISC_ $cdest _PNN>],MemoryReadError> {
                    Ok([<LD_MISC_ $cdest _PNN>] {
                        common: InstructionCommon::new(4, 20, true),
                        nn:memory.read_16(pos.wrapping_add(2))?,
                    })
                }

                pub fn new_with_value(nn: u16) -> [<LD_MISC_ $cdest _PNN>] {
                    [<LD_MISC_ $cdest _PNN>] {
                        common: InstructionCommon::new(4, 20, true),
                        nn,
                    }
                }
            }

            impl Display for [<LD_MISC_ $cdest _PNN>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, (0x{:04X})", $cdest, self.nn)
                }
            }

            impl BaseInstruction for [<LD_MISC_ $cdest _PNN>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    let nn_lsb = self.nn.to_le_bytes();
                    vec![0xed,hex!( $opcode )[0], nn_lsb[0], nn_lsb[1]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_MISC_ $cdest _PNN>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.[<$dest>] = memory.read_16(self.nn)?;
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use ld_misc_rr_pnn;
