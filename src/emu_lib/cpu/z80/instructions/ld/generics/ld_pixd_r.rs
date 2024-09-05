macro_rules! ld_pixd_r {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_PIXD_ $cdest>] {
                common: InstructionCommon,
                d: i8,
            }

            impl [<LD_PIXD_ $cdest>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_PIXD_ $cdest>],String> {
                    Ok([<LD_PIXD_ $cdest>] {
                        common: InstructionCommon::new(3, 19, true),
                        d:memory.read_8(pos.wrapping_add(2))? as i8,
                    })
                }

                pub fn new_with_value(d: u8) -> [<LD_PIXD_ $cdest>] {
                    [<LD_PIXD_ $cdest>] {
                        common: InstructionCommon::new(3, 19, true),
                        d: d as i8,
                    }
                }
            }

            impl Display for [<LD_PIXD_ $cdest>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD (IX+0x{:02X}), {}", self.d, $cdest)
                }
            }

            impl BaseInstruction for [<LD_PIXD_ $cdest>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![0xdd,hex!( $opcode )[0], self.d as u8]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_PIXD_ $cdest>] {
                fn runner(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    match cpu.registers.other.get("ix") {
            Some(BaseRegister::Bit16(ix)) => {
                let addr = ix.wrapping_add(self.d as u16);
                memory.write_8(addr, cpu.registers.gp[0].$dest)?;
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
            mod [<TEST_LD_PIXD_ $cdest>] {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!("dd",$opcode);

                test_instruction_parse!([<LD_PIXD_ $cdest>],[0x12]);
            }
        }
    }
}

pub(crate) use ld_pixd_r;
