use crate::cpu::i8080::I8080;
use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::InstructionParser;
use crate::memory::Memory;

#[derive(Debug, Default, Clone)]
pub struct I8080Parser {}

impl InstructionParser<I8080> for I8080Parser {
    fn ins_from_mem(
        &self,
        _memory: &Memory,
        _pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<I8080>)>, String> {
        unimplemented!()
    }
    fn ins_from_vec(
        &self,
        _memory: &Vec<u8>,
        _pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<I8080>)>, String> {
        unimplemented!()
    }
    fn ins_from_string(
        &self,
        _instruction: &str,
    ) -> Result<Box<(dyn ExecutableInstruction<I8080>)>, String> {
        unimplemented!()
    }
}
