macro_rules! xor_r {
        ($src:expr,$opcode:literal,$csrc:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<XOR_ $csrc>] {
                common: InstructionCommon,
            }

            impl [<XOR_ $csrc>] {
                pub fn new() -> [<XOR_ $csrc>] {
                    [<XOR_ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<XOR_ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "XOR {}", $csrc)
                }
            }

            impl BaseInstruction for [<XOR_ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<XOR_ $csrc>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let gp = &mut cpu.registers.gp;
                    xor_r_r_setf!(&mut gp.a,&mut gp.[<$src>], &mut gp.f);
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_XOR_ $csrc>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<XOR_ $csrc>]);
            }
        }
    }
}

pub(crate) use xor_r;
