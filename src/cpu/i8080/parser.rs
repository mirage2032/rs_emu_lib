use crate::cpu::i8080::I8080;
use crate::cpu::instruction::{ExecutableInstruction, ParseError};
use crate::cpu::InstructionParser;
use crate::memory::{MemoryDevice};

#[derive(Debug, Default, Clone)]
pub struct I8080Parser {}

impl InstructionParser<I8080> for I8080Parser {
    fn ins_from_asm_string(
        &self,
        _instruction: &str,
    ) -> Result<Box<(dyn ExecutableInstruction<I8080>)>, ParseError> {
        unimplemented!()
    }
    fn ins_from_machinecode(
        &self,
        _memory: &dyn MemoryDevice,
        _pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<I8080>)>, ParseError> {
        unimplemented!()
    }
}
