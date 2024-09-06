macro_rules! ld_r_n {
    ($dest:ident,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _N>] {
                common: InstructionCommon,
                n: u8,
            }

            impl [<LD_ $cdest _N>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_ $cdest _N>],String> {
                    Ok([<LD_ $cdest _N>] {
                        common: InstructionCommon::new(2, 7, true),
                        n:memory.read_8(pos.wrapping_add(1))?,
                    })
                }

                pub fn new_with_value(n: u8) -> [<LD_ $cdest _N>] {
                    [<LD_ $cdest _N>] {
                        common: InstructionCommon::new(2, 7, true),
                        n,
                    }
                }
            }

            impl Display for [<LD_ $cdest _N>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, 0x{:02x}", $cdest, self.n)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _N>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0], self.n]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _N>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = self.n;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_ $cdest _N>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_ $cdest _N>],[0x12]);
            }
        }
    }
}

pub(crate) use ld_r_n;
