macro_rules! adc_r_r {
        ($dest:expr,$src:expr,$opcode:literal,$cdest:literal,$csrc:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<ADC_ $cdest _ $csrc>] {
                common: InstructionCommon,
            }

            impl [<ADC_ $cdest _ $csrc>] {
                pub fn new() -> [<ADC_ $cdest _ $csrc>] {
                    [<ADC_ $cdest _ $csrc>] {
                        common: InstructionCommon::new(1, 4, true),
                    }
                }
            }

            impl Display for [<ADC_ $cdest _ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "ADC {}, {}", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<ADC_ $cdest _ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![hex!( $opcode )[0]]
                }
            }

            impl ExecutableInstruction<Z80> for [<ADC_ $cdest _ $csrc>] {
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    adc_r_r_setf!(
                        &mut cpu.registers.gp.[<$dest>],
                        cpu.registers.gp.[<$src>],
                        cpu.registers.gp.f
                    );
                    Ok(())
                }
            }

            #[allow(non_snake_case)]
            #[cfg(test)]
            mod [<TEST_ADC_ $cdest _ $csrc>] {
                use crate::cpu::test::*;
                use crate::cpu::z80::test::*;

                test_z80!($opcode);

                test_instruction_parse!([<ADC_ $cdest _ $csrc>]);
            }
        }
    }
}

pub(crate) use adc_r_r;