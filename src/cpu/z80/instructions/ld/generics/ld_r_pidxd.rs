macro_rules! ld_r_pidxd {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        // use crate::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_ $cdest _PIDXD>]<R: IndexRegister> {
                common: InstructionCommon,
                d: i8,
                index: PhantomData<R>,
            }

            pub type [<LD_ $cdest _PIXD>] = [<LD_ $cdest _PIDXD>]<IX>;
            pub type [<LD_ $cdest _PIYD>] = [<LD_ $cdest _PIDXD>]<IY>;

            impl<R: IndexRegister> [<LD_ $cdest _PIDXD>]<R> {
                pub fn new(memory: &dyn MemoryDevice, pos: u16) -> Result<Self,MemoryReadError> {
                    Ok(Self {
                        common: InstructionCommon::new(3, 19, true),
                        d:memory.read_8(pos.wrapping_add(2))? as i8,
                        index: PhantomData,
                    })
                }

                pub fn new_with_value(d: u8) -> Self {
                    Self {
                        common: InstructionCommon::new(3, 19, true),
                        d: d as i8,
                        index: PhantomData,
                    }
                }
            }

            impl<R: IndexRegister> Display for [<LD_ $cdest _PIDXD>]<R> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, ({name}+0x{:02X})", $cdest, self.d, name = R::NAME)
                }
            }

            impl<R: IndexRegister> BaseInstruction for [<LD_ $cdest _PIDXD>]<R> {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![R::PREFIX,hex!( $opcode )[0], self.d as u8]
                }
            }

            impl<R: IndexRegister> ExecutableInstruction<Z80> for [<LD_ $cdest _PIDXD>]<R> {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    let addr = R::get(&cpu.registers).wrapping_add(self.d as u16);
                    cpu.registers.gp.$dest = memory.read_8(addr)?;
                    Ok(())
                }
            }
        }
    }
}

pub(crate) use ld_r_pidxd;
