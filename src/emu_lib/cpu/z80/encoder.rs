use regex::Regex;

use crate::emu_lib::cpu::{ExecutableInstruction, InstructionEncoder};
use crate::emu_lib::cpu::z80::instructions::{ex, halt, ld, math, nop, rlca, rrca};
use crate::emu_lib::cpu::z80::Z80;

enum ImmediateValue {
    Val(Number),
    Ptr(Number),
}

enum Number {
    U8(u8),
    U16(u16),
}

fn is_num(number: &str) -> Result<Number, String> {
    let num = if number.starts_with("0x") && number.len() <= 4 {
        Number::U8(u8::from_str_radix(&number[2..], 16).map_err(|e| e.to_string())?)
    } else if number.starts_with("0b") && number.len() <= 10 {
        Number::U8(u8::from_str_radix(&number[2..], 2).map_err(|e| e.to_string())?)
    } else if number.starts_with("0x") && number.len() <= 6 {
        Number::U16(u16::from_str_radix(&number[1..], 8).map_err(|e| e.to_string())?)
    } else if number.starts_with("0b") && number.len() <= 18 {
        Number::U16(u16::from_str_radix(&number[1..], 10).map_err(|e| e.to_string())?)
    } else if number.len() <= 3 {
        Number::U8(u8::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    } else if number.len() <= 5 {
        Number::U16(u16::from_str_radix(&number, 10).map_err(|e| e.to_string())?)
    } else {
        return Err("Invalid number".to_string());
    };
    Ok(num)
}

fn is_val(number: &str) -> Result<ImmediateValue, String> {
    if number.starts_with("(") && number.ends_with(")") {
        let parsed = &number[1..number.len() - 1];
        Ok(ImmediateValue::Ptr(is_num(parsed)?))
    } else {
        Ok(ImmediateValue::Val(is_num(number)?))
    }
}

impl InstructionEncoder for Z80 {
    fn encode(instruction: String) -> Result<Box<(dyn ExecutableInstruction<Self>)>, String> {
        let filtered = instruction.to_lowercase().replace(",", " ");
        //regex
        let re = Regex::new(r"^([a-z]+)(?: +([(a-z0-9')]+)(?: ?+,? ?+([(a-z0-9')]+))?)?$").unwrap();
        let op = re.captures(&filtered).expect("Invalid instruction");
        let instruction: Box<dyn ExecutableInstruction<Self>> = match op.get(1).unwrap().as_str() {
            "nop" => Box::new(nop::NOP::new()),
            "ld" => {
                let destination = op.get(2).unwrap().as_str();
                let source = op.get(3).unwrap().as_str();
                match (is_val(destination), is_val(source)) {
                    (Err(_), Ok(ImmediateValue::Val(Number::U16(val)))) =>
                        match destination {
                            "bc" => Box::new(ld::ld_bc_nn::LD_BC_NN::new_with_value(val)),
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" destination register {0}", destination, source))
                        },
                    (Err(_), Ok(ImmediateValue::Val(Number::U8(val)))) =>
                        match destination {
                            "b" => Box::new(ld::ld_b_n::LD_B_N::new_with_value(val)),
                            "c" => Box::new(ld::ld_c_n::LD_C_N::new_with_value(val)),
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" destination register {0}", destination, source))
                        },
                    (Err(_), Ok(ImmediateValue::Ptr(Number::U16(_)))) =>
                        match destination {
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" destination register {0}", destination, source))
                        },
                    (Err(_), Ok(ImmediateValue::Ptr(Number::U8(_)))) =>
                        match destination {
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" destination register {0}", destination, source))
                        },

                    (Ok(ImmediateValue::Val(_)), Err(_)) =>
                        return Err(format!("Invalid \"ld {0}, {1}\" source register {1}", destination, source)),
                    (Ok(ImmediateValue::Ptr(Number::U16(_))), Err(_)) =>
                        match source {
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" source register {1}", destination, source))
                        },
                    (Ok(ImmediateValue::Ptr(Number::U8(_))), Err(_)) =>
                        match source {
                            _ => return Err(format!("Invalid \"ld {0}, {1}\" source register {1}", destination, source))
                        },

                    (Ok(_), Ok(_)) => {
                        return Err("Invalid operands".to_string());
                    }
                    (Err(_), Err(_)) => {
                        match (destination, source) {
                            ("(bc)", "a") => Box::new(ld::ld_pbc_a::LD_PBC_A::new()),
                            ("a", "(bc)") => Box::new(ld::ld_a_pbc::LD_A_PBC::new()),
                            _ => return Err("Invalid operands".to_string())
                        }
                    }
                }
            }
            "inc" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "bc" => Box::new(math::inc::inc_bc::INC_BC::new()),
                    "b" => Box::new(math::inc::inc_b::INC_B::new()),
                    "c" => Box::new(math::inc::inc_c::INC_C::new()),
                    _ => return Err("Invalid instruction".to_string())
                }
            }
            "dec" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "bc" => Box::new(math::dec::dec_bc::DEC_BC::new()),
                    "b" => Box::new(math::dec::dec_b::DEC_B::new()),
                    "c" => Box::new(math::dec::dec_c::DEC_C::new()),
                    _ => return Err("Invalid instruction".to_string())
                }
            }
            "add" => {
                let destination = op.get(2).unwrap().as_str();
                match destination {
                    "hl" => {
                        let source = op.get(3).unwrap().as_str();
                        match source {
                            "bc" => Box::new(math::add::add_hl_bc::ADD_HL_BC::new()),
                            _ => return Err("Invalid source".to_string())
                        }
                    }
                    _ => return Err("Invalid destination".to_string())
                }
            }
            "rlca" => Box::new(rlca::RLCA::new()),
            "ex" => {
                let op1 = op.get(2).unwrap().as_str();
                let op2 = op.get(3).unwrap().as_str();
                match (op1, op2) {
                    ("af", "af'") => Box::new(ex::ex_af_saf::EX_AF_SAF::new()),
                    _ => return Err("Invalid operands".to_string())
                }
            }
            "rrca" => Box::new(rrca::RRCA::new()),
            "halt" => Box::new(halt::Halt::new()),
            _ => return Err("Invalid instruction".to_string())
        };
        Ok(instruction)
    }
}