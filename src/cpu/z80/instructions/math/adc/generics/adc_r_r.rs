macro_rules! adc_r_r {
        ($dest:expr,$src:expr,$opcode:literal,$cdest:literal,$csrc:literal) => {
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
                    let gp = &mut cpu.registers.gp;
                    let carry = gp.f.carry();
                    gp.[<$dest>] = alu::add8(&mut gp.f, gp.[<$dest>], gp.[<$src>], carry);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use adc_r_r;