macro_rules! ld_pidxd_r {
    ($dest: expr ,$opcode:literal,$cdest:literal) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<LD_PIDXD_ $cdest>]<R: IndexRegister> {
                common: InstructionCommon,
                d: i8,
                index: PhantomData<R>,
            }

            pub type [<LD_PIXD_ $cdest>] = [<LD_PIDXD_ $cdest>]<IX>;
            pub type [<LD_PIYD_ $cdest>] = [<LD_PIDXD_ $cdest>]<IY>;

            impl<R: IndexRegister> [<LD_PIDXD_ $cdest>]<R> {
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

            impl<R: IndexRegister> Display for [<LD_PIDXD_ $cdest>]<R> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD ({name}+0x{:02X}), {}", self.d, $cdest, name = R::NAME)
                }
            }

            impl<R: IndexRegister> BaseInstruction for [<LD_PIDXD_ $cdest>]<R> {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![R::PREFIX,hex!( $opcode )[0], self.d as u8]
                }
            }

            impl<R: IndexRegister> ExecutableInstruction<Z80> for [<LD_PIDXD_ $cdest>]<R> {
                fn execute(&mut self, memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {

                let addr = R::get(&cpu.registers).wrapping_add(self.d as u16);
                memory.write_8(addr, cpu.registers.gp.$dest)?;
        Ok(())
                }
            }
        }
    }
}

pub(crate) use ld_pidxd_r;
