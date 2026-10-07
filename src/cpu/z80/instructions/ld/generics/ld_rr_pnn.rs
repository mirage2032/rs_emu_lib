macro_rules! ld_rr_pnn {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _PNN>] {
                common: InstructionCommon,
                nn: u16,
            }

            impl [<LD_ $cdest _PNN>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_ $cdest _PNN>],MemoryReadError> {
                    Ok(Self::new_with_value(memory.read_16(pos.wrapping_add(1))?))
                }

                pub fn new_with_value(nn: u16) -> [<LD_ $cdest _PNN>] {
                    [<LD_ $cdest _PNN>] {
                        common: InstructionCommon::new(3, 16, true),
                        nn,
                    }
                }
            }

            impl Display for [<LD_ $cdest _PNN>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, (0x{:04X})", $cdest, self.nn)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _PNN>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    let nn_lsb = self.nn.to_le_bytes();
                    vec![hex!( $opcode )[0], nn_lsb[0], nn_lsb[1]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _PNN>] {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.[<$dest>] = memory.read_16(self.nn)?;
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use ld_rr_pnn;
