use regex::Regex;

use crate::emu_lib::cpu::instruction::{BaseInstruction, ExecutableInstruction, InstructionParser};
use crate::emu_lib::cpu::z80::instructions::*;
use crate::emu_lib::cpu::z80::Z80;
use crate::memory::{memdevices::ROM, Memory, MemoryDevice};

#[derive(Debug, Clone)]
enum ImmediateValue {
    Val8(u8),
    Val16(u16),
    Ptr(u16),
}

// #[derive(Debug, Clone)]
// enum Number {
//     U8(u8),
//     U16(u16),
// }

fn is_num(number: &str) -> Result<u16, String> {
    let num = if number.starts_with("0x") && number.len() <= 6 {
        u16::from_str_radix(&number[2..], 16).map_err(|e| e.to_string())?
    } else if number.starts_with("0b") && number.len() <= 18 {
        u16::from_str_radix(&number[2..], 2).map_err(|e| e.to_string())?
    // } else if number.len() <= 3 {
    //     Number::U8(u8::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    // } else if number.len() <= 5 {
    //     Number::U16(u16::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    } else {
        u16::from_str_radix(&number, 10).map_err(|e| e.to_string())?
    };
    Ok(num)
}

fn is_val(number: &str) -> Result<ImmediateValue, String> {
    if number.starts_with("(") && number.ends_with(")") {
        let parsed = &number[1..number.len() - 1];
        Ok(ImmediateValue::Ptr(is_num(parsed)?))
    } else {
        let num = is_num(number)?;
        if num <= 0xFF {
            Ok(ImmediateValue::Val8(num as u8))
        } else {
            Ok(ImmediateValue::Val16(num))
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Z80Parser {}

impl Z80Parser {
    pub fn from_memdev(
        memory: &dyn MemoryDevice,
        pos: u16,
    ) -> Result<Box<(dyn ExecutableInstruction<Z80>)>, String> {
        let ins_byte0 = memory.read_8(pos)?;
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match ins_byte0 {
            0x00u8 => Box::new(nop::NOP::new()),
            0x01 => Box::new(ld::ld_bc_nn::LD_BC_NN::new(memory, pos)?),
            0x02 => Box::new(ld::ld_pbc_a::LD_PBC_A::new()),
            0x03 => Box::new(math::inc::inc_bc::INC_BC::new()),
            0x04 => Box::new(math::inc::inc_b::INC_B::new()),
            0x05 => Box::new(math::dec::dec_b::DEC_B::new()),
            0x06 => Box::new(ld::ld_b_n::LD_B_N::new(memory, pos)?),
            0x07 => Box::new(rlca::RLCA::new()),
            0x08 => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
            0x09 => Box::new(math::add::add_hl_bc::ADD_HL_BC::new()),
            0x0A => Box::new(ld::ld_a_pbc::LD_A_PBC::new()),
            0x0B => Box::new(math::dec::dec_bc::DEC_BC::new()),
            0x0C => Box::new(math::inc::inc_c::INC_C::new()),
            0x0D => Box::new(math::dec::dec_c::DEC_C::new()),
            0x0E => Box::new(ld::ld_c_n::LD_C_N::new(memory, pos)?),
            0x0F => Box::new(rrca::RRCA::new()),
            0x10 => Box::new(djnz_d::DJNZ_D::new(memory, pos)?),
            0x11 => Box::new(ld::ld_de_nn::LD_DE_NN::new(memory, pos)?),
            0x12 => Box::new(ld::ld_pde_a::LD_PDE_A::new()),
            0x13 => Box::new(math::inc::inc_de::INC_DE::new()),
            0x14 => Box::new(math::inc::inc_d::INC_D::new()),
            0x15 => Box::new(math::dec::dec_d::DEC_D::new()),
            0x16 => Box::new(ld::ld_d_n::LD_D_N::new(memory, pos)?),
            0x17 => Box::new(rla::RLA::new()),
            0x18 => Box::new(jump::jr::jr_d::JR_D::new(memory, pos)?),
            0x19 => Box::new(math::add::add_hl_de::ADD_HL_DE::new()),
            0x1A => Box::new(ld::ld_a_pde::LD_A_PDE::new()),
            0x1B => Box::new(math::dec::dec_de::DEC_DE::new()),
            0x1C => Box::new(math::inc::inc_e::INC_E::new()),
            0x1D => Box::new(math::dec::dec_e::DEC_E::new()),
            0x1E => Box::new(ld::ld_e_n::LD_E_N::new(memory, pos)?),
            0x1F => Box::new(rra::RRA::new()),
            0x20 => Box::new(jump::jr::jr_nz_d::JR_NZ_D::new(memory, pos)?),
            0x21 => Box::new(ld::ld_hl_nn::LD_HL_NN::new(memory, pos)?),
            0x23 => Box::new(math::inc::inc_hl::INC_HL::new()),
            0x2A => Box::new(ld::ld_hl_pnn::LD_HL_PNN::new(memory, pos)?),
            0x30 => Box::new(jump::jr::jr_nc_d::JR_NC_D::new(memory, pos)?),
            0x36 => Box::new(ld::ld_phl_n::LD_PHL_N::new(memory, pos)?),
            0x76 => Box::new(halt::Halt::new()),
            0x79 => Box::new(ld::ld_a_c::LD_A_C::new()),
            0xC5 => Box::new(stack::push::push_bc::PUSH_BC::new()),
            0xC9 => Box::new(ret::RET::new()),
            0xCD => Box::new(call::call_nn::CALL_NN::new(memory, pos)?),
            0xD6 => Box::new(math::sub::sub_n::SUB_N::new(memory, pos)?),
            _ => return Err(format!("Invalid instruction: 0x{:02x}", ins_byte0)),
        };
        Ok(instruction)
    }
    pub fn from_string(
        instruction: &String,
    ) -> Result<Box<(dyn ExecutableInstruction<Z80>)>, String> {
        let filtered = instruction.to_lowercase().replace(",", " ");
        //regex
        let re = Regex::new(r"^([a-z]+)(?: +([(a-z0-9')]+)(?: ?+,? ?+([(a-z0-9')]+))?)?$")
            .expect("Error building Z80 instruction parsing regex");
        let op = re.captures(&filtered).expect("Invalid instruction");
        let instruction: Box<dyn ExecutableInstruction<Z80>> = match op.get(1).unwrap().as_str() {
            "nop" => Box::new(nop::NOP::new()),
            "ld" => {
                let destination = op.get(2).unwrap().as_str();
                let source = op.get(3).unwrap().as_str();
                let _d = is_val(destination);
                let _s = is_val(source);
                match (is_val(destination), is_val(source)) {
                    (Err(_), Ok(ImmediateValue::Val8(val))) => match destination {
                        "b" => Box::new(ld::ld_b_n::LD_B_N::new_with_value(val)),
                        "c" => Box::new(ld::ld_c_n::LD_C_N::new_with_value(val)),
                        "d" => Box::new(ld::ld_d_n::LD_D_N::new_with_value(val)),
                        "e" => Box::new(ld::ld_e_n::LD_E_N::new_with_value(val)),
                        "(hl)" => Box::new(ld::ld_phl_n::LD_PHL_N::new_with_value(val)),
                        _ => {
                            return Err(format!(
                                "Invalid \"ld {0}, {1}\" destination register {0}",
                                destination, source
                            ))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::Val16(val))) => match destination {
                        "bc" => Box::new(ld::ld_bc_nn::LD_BC_NN::new_with_value(val)),
                        "de" => Box::new(ld::ld_de_nn::LD_DE_NN::new_with_value(val)),
                        "hl" => Box::new(ld::ld_hl_nn::LD_HL_NN::new_with_value(val)),
                        _ => {
                            return Err(format!(
                                "Invalid \"ld {0}, {1}\" destination register {0}",
                                destination, source
                            ))
                        }
                    },
                    (Err(_), Ok(ImmediateValue::Ptr(val))) => match destination {
                        "hl" => Box::new(ld::ld_hl_pnn::LD_HL_PNN::new_with_value(val)),
                        _ => {
                            return Err(format!(
                                "Invalid \"ld {0}, {1}\" destination register {0}",
                                destination, source
                            ))
                        }
                    },

                    (Ok(ImmediateValue::Val16(_)), Err(_)) => {
                        return Err(format!(
                            "Invalid \"ld {0}, {1}\" source register {1}",
                            destination, source
                        ))
                    }
                    (Ok(ImmediateValue::Ptr(_)), Err(_)) => match source {
                        _ => {
                            return Err(format!(
                                "Invalid \"ld {0}, {1}\" source register {1}",
                                destination, source
                            ))
                        }
                    },
                    // (Ok(ImmediateValue::Ptr(_)), Err(_)) =>
                    //     match source {
                    //         _ => return Err(format!("Invalid \"ld {0}, {1}\" source register {1}", destination, source))
                    //     },
                    (Ok(_), Ok(_)) => {
                        return Err("Invalid operands".to_string());
                    }
                    (Err(_), Err(_)) => match (destination, source) {
                        ("(bc)", "a") => Box::new(ld::ld_pbc_a::LD_PBC_A::new()),
                        ("a", "(bc)") => Box::new(ld::ld_a_pbc::LD_A_PBC::new()),
                        ("(de)", "a") => Box::new(ld::ld_pde_a::LD_PDE_A::new()),
                        ("a", "(de)") => Box::new(ld::ld_a_pde::LD_A_PDE::new()),
                        ("a", "c") => Box::new(ld::ld_a_c::LD_A_C::new()),
                        _ => return Err("Invalid operands".to_string()),
                    },
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "inc" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "bc" => Box::new(math::inc::inc_bc::INC_BC::new()),
                    "de" => Box::new(math::inc::inc_de::INC_DE::new()),
                    "hl" => Box::new(math::inc::inc_hl::INC_HL::new()),
                    "b" => Box::new(math::inc::inc_b::INC_B::new()),
                    "c" => Box::new(math::inc::inc_c::INC_C::new()),
                    "d" => Box::new(math::inc::inc_d::INC_D::new()),
                    "e" => Box::new(math::inc::inc_e::INC_E::new()),
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "dec" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "bc" => Box::new(math::dec::dec_bc::DEC_BC::new()),
                    "de" => Box::new(math::dec::dec_de::DEC_DE::new()),
                    "b" => Box::new(math::dec::dec_b::DEC_B::new()),
                    "c" => Box::new(math::dec::dec_c::DEC_C::new()),
                    "d" => Box::new(math::dec::dec_d::DEC_D::new()),
                    "e" => Box::new(math::dec::dec_e::DEC_E::new()),
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "add" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "hl" => {
                        let source = op.get(3).unwrap().as_str();
                        match source {
                            "bc" => Box::new(math::add::add_hl_bc::ADD_HL_BC::new()),
                            "de" => Box::new(math::add::add_hl_de::ADD_HL_DE::new()),
                            _ => return Err("Invalid source".to_string()),
                        }
                    }
                    _ => return Err("Invalid destination".to_string()),
                }
            }
            "sub" => {
                let destination = op.get(2).unwrap().as_str();
                match is_val(destination) {
                    Ok(ImmediateValue::Val8(val)) => Box::new(math::sub::sub_n::SUB_N::new_with_value(val)),
                    _ => return Err("Invalid destination".to_string()),
                }
            }
            "rlca" => Box::new(rlca::RLCA::new()),
            "ex" => {
                let op1 = op.get(2).unwrap().as_str();
                let op2 = op.get(3).unwrap().as_str();
                match (op1, op2) {
                    ("af", "af'") => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
                    _ => return Err("Invalid operands".to_string()),
                }
            }
            "rrca" => Box::new(rrca::RRCA::new()),
            "djnz" => {
                let destination = is_val(op.get(2).unwrap().as_str());
                match destination {
                    Ok(ImmediateValue::Val8(val)) => Box::new(djnz_d::DJNZ_D::new_with_value(val)),
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "rla" => Box::new(rla::RLA::new()),
            "jr" => {
                let op1 = op.get(2);
                let op2 = op.get(3);
                match (op1, op2) {
                    (Some(op1_match), None) => {
                        let op1_val = is_val(op1_match.as_str());
                        match op1_val {
                            Ok(ImmediateValue::Val8(val)) => {
                                Box::new(jump::jr::jr_d::JR_D::new_with_value(val))
                            }
                            _ => return Err("Invalid instruction".to_string()),
                        }
                    }
                    (Some(op1_match), Some(op2_match)) => {
                        let op2_val = is_val(op2_match.as_str());
                        match (op1_match.as_str(), op2_val) {
                            ("nz", Ok(ImmediateValue::Val8(val))) => Box::new(jump::jr::jr_nz_d::JR_NZ_D::new_with_value(val)),
                            ("nc", Ok(ImmediateValue::Val8(val))) => Box::new(jump::jr::jr_nc_d::JR_NC_D::new_with_value(val)),
                            _ => return Err("Invalid instruction".to_string()),
                        }
                    }
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "rra" => Box::new(rra::RRA::new()),
            "halt" => Box::new(halt::Halt::new()),
            "call" => {
                let destination = is_val(op.get(2).unwrap().as_str());
                match destination {
                    Ok(ImmediateValue::Val16(val)) => {
                        Box::new(call::call_nn::CALL_NN::new_with_value(val))
                    }
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            "ret" => Box::new(ret::RET::new()),
            "push" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "bc" => Box::new(stack::push::push_bc::PUSH_BC::new()),
                    _ => return Err("Invalid instruction".to_string()),
                }
            }
            _ => return Err("Invalid instruction".to_string()),
        };
        Ok(instruction)
    }
}

impl InstructionParser for Z80Parser {
    fn ins_from_mem(
        &self,
        memory: &Memory,
        pos: u16,
    ) -> Result<Box<(dyn BaseInstruction)>, String> {
        Z80Parser::from_memdev(memory, pos).map(|x| x as Box<dyn BaseInstruction>)
    }
    fn ins_from_vec(
        &self,
        memory: Vec<u8>,
        pos: u16,
    ) -> Result<Box<(dyn BaseInstruction)>, String> {
        let rom: ROM = memory.into();
        Z80Parser::from_memdev(&rom, pos).map(|x| x as Box<dyn BaseInstruction>)
    }
    fn ins_from_string(&self, instruction: &String) -> Result<Box<(dyn BaseInstruction)>, String> {
        Z80Parser::from_string(instruction).map(|x| x as Box<dyn BaseInstruction>)
    }
}
