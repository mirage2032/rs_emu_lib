macro_rules! ld_prr_n {
    ($dest:ident,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_P $cdest _N>] {
                common: InstructionCommon,
                n: u8,
            }

            impl [<LD_P $cdest _N>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_P $cdest _N>],MemoryReadError> {
                    Ok([<LD_P $cdest _N>] {
                        common: InstructionCommon::new(2, 10, true),
                        n:memory.read_8(pos.wrapping_add(1))?,
                    })
                }

                pub fn new_with_value(n: u8) -> [<LD_P $cdest _N>] {
                    [<LD_P $cdest _N>] {
                        common: InstructionCommon::new(2, 10, true),
                        n,
                    }
                }
            }

            impl Display for [<LD_P $cdest _N>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD ({}), 0x{:02x}", $cdest, self.n)
                }
            }

            impl BaseInstruction for [<LD_P $cdest _N>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0], self.n]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_P $cdest _N>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    memory.write_8(cpu.registers.gp.[<$dest>], self.n)?;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_P $cdest _N>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_P $cdest _N>],[0x12]);
            }
        }
    }
}

pub(crate) use ld_prr_n;
