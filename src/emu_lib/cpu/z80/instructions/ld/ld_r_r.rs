macro_rules! ld_r_r {
    ($dest:ident,$src:ident,$opcode:literal,$cdest:literal,$csrc:literal) => {
        use core::fmt;
        use std::fmt::Display;

        use once_cell::sync::Lazy;

        use crate::emu_lib::cpu::instruction::InstructionCommon;
        use crate::emu_lib::cpu::BaseInstruction;
        use crate::cpu::z80::ExecutableInstruction;
        use crate::cpu::z80::Z80;
        use crate::memory::Memory;
        use crate::io::IO;

        static COMMON: Lazy<InstructionCommon> = Lazy::new(|| InstructionCommon::new(1, 4, true));

        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::item! {
            const [<opcode_ $opcode _num>]: u8 = u8::from_str_radix($opcode, 16).unwrap();
            #[derive(Debug)]
            pub struct [<LD_ $cdest _ $csrc>] {
                common: InstructionCommon,
            }

            impl [<LD_ $cdest _ $csrc>] {
                pub fn new() -> [<LD_ $cdest _ $csrc>] {
                    [<LD_ $cdest _ $csrc>] {
                        common: *COMMON,
                    }
                }
            }

            impl Display for [<LD_ $cdest _ $csrc>] {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "LD {}, {}", $cdest, $csrc)
                }
            }

            impl BaseInstruction for [<LD_ $cdest _ $csrc>] {
                fn common(&self) -> &InstructionCommon {
                    &self.common
                }
                fn to_bytes(&self) -> Vec<u8> {
                    vec![[<opcode_ $opcode _num>]]
                }
            }

            impl ExecutableInstruction<Z80> for [<LD_ $cdest _ $csrc>] {
                fn runner(&mut self, _memory: &mut Memory, cpu: &mut Z80, _: &mut IO) -> Result<(), String> {
                    cpu.registers.gp[0].[<$dest>] = cpu.registers.gp[0].[<$src>];
                    Ok(())
                }
            }

            #[cfg(test)]
            mod tests {
                use crate::emu_lib::cpu::test::*;
                use crate::emu_lib::cpu::z80::test::*;

                test_z80!([<"4f">]);

                test_instruction_parse!([<LD_ $cdest _ $csrc>]);
            }
        }
    }
}

pub(crate) use ld_r_r;
