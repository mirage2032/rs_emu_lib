macro_rules! bit_b_pidxd {
        ($bit:literal,$opcode:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<BIT_ $bit _PIDXD>]<R: IndexRegister> {
                common: InstructionCommon,
                d: i8,
                index: PhantomData<R>,
            }

            pub type [<BIT_ $bit _PIXD>] = [<BIT_ $bit _PIDXD>]<IX>;
            pub type [<BIT_ $bit _PIYD>] = [<BIT_ $bit _PIDXD>]<IY>;

            impl<R: IndexRegister> [<BIT_ $bit _PIDXD>]<R> {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self, MemoryReadError> {
                    Ok(Self::new_with_value(memory.read_8(pos.wrapping_add(2))?))
                }
                pub fn new_with_value(d: u8) -> Self {
                    Self {
                        common: InstructionCommon::new(4, 20, true),
                        d: d as i8,
                        index: PhantomData,
                        }
                }
            }

            impl<R: IndexRegister> Display for [<BIT_ $bit _PIDXD>]<R> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "BIT {}, ({name}+0x{:02X})",$bit , self.d, name = R::NAME)
                }
            }

            impl<R: IndexRegister> BaseInstruction for [<BIT_ $bit _PIDXD>]<R> {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![R::PREFIX,0xcb,self.d as u8,hex!( $opcode )[0]]
                }
            }

            impl<R: IndexRegister> ExecutableInstruction<Z80> for [<BIT_ $bit _PIDXD>]<R> {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let offset = R::get(&cpu.registers).wrapping_add(self.d as u16);
                    let value = memory.read_8(offset as u16)?;
                    alu::bit(&mut cpu.registers.gp.f, $bit, value, Some((offset >> 8) as u8));
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use bit_b_pidxd;
