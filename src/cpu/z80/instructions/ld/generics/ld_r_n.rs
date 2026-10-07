macro_rules! ld_r_n {
    ($dest:ident,$opcode:literal,$cdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _N>] {
                common: InstructionCommon,
                n: u8,
            }

            impl [<LD_ $cdest _N>] {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<[<LD_ $cdest _N>],MemoryReadError> {
                    Ok(Self::new_with_value(memory.read_8(pos.wrapping_add(1))?))
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
                    write!(f, "LD {}, 0x{:02X}", $cdest, self.n)
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
                fn execute(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp.[<$dest>] = self.n;
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use ld_r_n;
