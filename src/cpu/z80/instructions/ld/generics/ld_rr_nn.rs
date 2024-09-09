macro_rules! ld_rr_nn {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _NN>] {
                common: InstructionCommon,
                nn: u16,
            }

            impl [<LD_ $cdest _NN>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_ $cdest _NN>],String> {
                    Ok([<LD_ $cdest _NN>] {
                        common: InstructionCommon::new(3, 10, true),
                        nn:memory.read_16(pos.wrapping_add(1))?,
                    })
                }

                pub fn new_with_value(nn: u16) -> [<LD_ $cdest _NN>] {
                    [<LD_ $cdest _NN>] {
                        common: InstructionCommon::new(3, 10, true),
                        nn,
                    }
                }
            }

            impl Display for [<LD_ $cdest _NN>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, 0x{:04x}", $cdest, self.nn)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _NN>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    let nn_lsb = self.nn.to_le_bytes();
                    vec![hex!( $opcode )[0], nn_lsb[0], nn_lsb[1]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _NN>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.[<$dest>] = self.nn;
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_LD_ $cdest _NN>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<LD_ $cdest _NN>],[0x12]);
            }
        }
    }
}

pub(crate) use ld_rr_nn;
