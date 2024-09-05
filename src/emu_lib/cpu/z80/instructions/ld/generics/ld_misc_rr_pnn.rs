macro_rules! ld_misc_rr_pnn {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::item! {
            #[derive(Debug)]
            pub struct [<LD_MISC_ $cdest _PNN>] {
                common: InstructionCommon,
                nn: u16,
            }

            impl [<LD_MISC_ $cdest _PNN>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_MISC_ $cdest _PNN>],String> {
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
                    write!(f, "LD {}, ({:04})", $cdest, self.nn)
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
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = memory.read_16(self.nn)?;
                    match cpu.registers.other.get_mut("r") {
            Some(BaseRegister::Bit8(val)) => {
                *val = val.wrapping_add(1) % 128;
            }
            _ => return Err("Invalid register".to_string()),
        }
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_MISC_ $cdest _PNN>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;
                test_z80!("ed",$opcode);

                test_instruction_parse!([<LD_MISC_ $cdest _PNN>],[0x1234]);
            }
        }
    }
}

pub(crate) use ld_misc_rr_pnn;
