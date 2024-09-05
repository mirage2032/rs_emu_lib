macro_rules! ld_r_pixd {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _PIXD>] {
                common: InstructionCommon,
                d: i8,
            }

            impl [<LD_ $cdest _PIXD>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_ $cdest _PIXD>],String> {
                    Ok([<LD_ $cdest _PIXD>] {
                        common: InstructionCommon::new(3, 19, true),
                        d:memory.read_8(pos.wrapping_add(2))? as i8,
                    })
                }

                pub fn new_with_value(d: u8) -> [<LD_ $cdest _PIXD>] {
                    [<LD_ $cdest _PIXD>] {
                        common: InstructionCommon::new(3, 19, true),
                        d: d as i8,
                    }
                }
            }

            impl Display for [<LD_ $cdest _PIXD>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, (IX+0x{:02X})", $cdest, self.d)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _PIXD>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xdd,hex!( $opcode )[0], self.d as u8]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _PIXD>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(ix)) => {
                let addr = ix.wrapping_add(self.d as u16);
                cpu.registers.gp[0].$dest = memory.read_8(addr)?;
            }
            _ => {
                return Err("IX register not found".to_string());
            }
        }
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
            mod [<TEST_LD_ $cdest _PIXD>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!("dd",$opcode);

                test_instruction_parse!([<LD_ $cdest _PIXD>],[0x12]);
            }
        }
    }
}

pub(crate) use ld_r_pixd;
