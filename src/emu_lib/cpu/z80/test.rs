use serde::{Deserialize, Serialize};

use crate::cpu::CPUType;
use crate::cpu::registers::BaseRegister;
use crate::emulator::Emulator;
use crate::memory::{Memory, MemoryDevice};
use crate::memory::memdevices::RAM;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestState {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    i: u8,
    r: u8,
    ei: u8,
    wz: u16,
    ix: u16,
    iy: u16,
    af_: u16,
    bc_: u16,
    de_: u16,
    hl_: u16,
    im: u8,
    iff1: u8,
    iff2: u8,
    ram: Vec<(u16, u8)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestData {
    name: String,
    initial: TestState,
    #[serde(rename = "final")]
    final_state: TestState,
    cycles: Vec<(u16, Option<u8>, String)>,
}

fn setup_z80(emulator: &mut Emulator, state: &TestState) -> Result<(), &'static str> {
    let registers = emulator.cpu.registers_mut();
    registers.pc = state.pc;
    registers.sp = state.sp;
    registers.gp[0].a = state.a;
    registers.gp[0].b = state.b;
    registers.gp[0].c = state.c;
    registers.gp[0].d = state.d;
    registers.gp[0].e = state.e;
    registers.gp[0].f = state.f.into();
    registers.gp[0].h = state.h;
    registers.gp[0].l = state.l;
    registers.other.insert("i", BaseRegister::Bit8(state.i));
    registers.other.insert("r", BaseRegister::Bit8(state.r));
    //registers.other.insert("ei",BaseRegister::Bit8(state.ei));
    //registers.other.insert("wz",BaseRegister::Bit16(state.wz));
    registers.other.insert("ix", BaseRegister::Bit16(state.ix));
    registers.other.insert("iy", BaseRegister::Bit16(state.iy));
    registers.gp[1].af = state.af_;
    registers.gp[1].bc = state.bc_;
    registers.gp[1].de = state.de_;
    registers.gp[1].hl = state.hl_;
    //registers.other.insert("im",BaseRegister::Bit8(state.im));
    //registers.other.insert("p",BaseRegister::Bit16(state.p));
    //registers.other.insert("q",BaseRegister::Bit16(state.q));
    emulator.io.iff1 = if state.iff1 == 1 { true } else { false };
    emulator.io.iff2 = if state.iff2 == 1 { true } else { false };
    for (address, value) in &state.ram {
        emulator.memory.write_8(*address, *value)?;
    }
    Ok(())
}

fn assert_z80(emulator: &mut Emulator, test_state: &TestState) {
    let registers = emulator.cpu.registers();
    assert_eq!(registers.gp[0].a, test_state.a);
    assert_eq!(registers.gp[0].b, test_state.b);
    assert_eq!(registers.gp[0].c, test_state.c);
    assert_eq!(registers.gp[0].d, test_state.d);
    assert_eq!(registers.gp[0].e, test_state.e);
    assert_eq!(registers.gp[0].f, test_state.f.into());
    assert_eq!(registers.gp[0].h, test_state.h);
    assert_eq!(registers.gp[0].l, test_state.l);
    assert_eq!(registers.other["i"], BaseRegister::Bit8(test_state.i));
    assert_eq!(registers.other["r"], BaseRegister::Bit8(test_state.r));
    assert_eq!(registers.gp[1].af, test_state.af_);
    assert_eq!(registers.gp[1].bc, test_state.bc_);
    assert_eq!(registers.gp[1].de, test_state.de_);
    assert_eq!(registers.gp[1].hl, test_state.hl_);
    assert_eq!(registers.other["ix"], BaseRegister::Bit16(test_state.ix));
    assert_eq!(registers.other["iy"], BaseRegister::Bit16(test_state.iy));
    assert_eq!(registers.pc, test_state.pc);
    assert_eq!(registers.sp, test_state.sp);
    //assert_eq!(registers.other["wz"],BaseRegister::Bit16(test_state.wz));
    assert_eq!(
        emulator.io.iff1,
        if test_state.iff1 == 1 { true } else { false }
    );
    assert_eq!(
        emulator.io.iff2,
        if test_state.iff2 == 1 { true } else { false }
    );
    //assert_eq!(registers.other["im"],BaseRegister::Bit8(test_state.im));
    //assert_eq!(registers.other["ei"],BaseRegister::Bit8(test_state.ei));
    //assert_eq!(registers.other["p"],BaseRegister::Bit16(test_state.p));
    //assert_eq!(registers.other["q"],BaseRegister::Bit16(test_state.q));
    for (address, value) in &test_state.ram {
        assert_eq!(emulator.memory.read_8(*address).unwrap(), *value);
    }
}

pub fn test_z80_w_data(test_data_vec: Vec<TestData>) {
    for test_data in test_data_vec {
        let mut memory = Memory::new();
        let rom = RAM::new(0x10000);
        memory.add_device(Box::new(rom));
        let mut emulator = Emulator::new_w_mem(CPUType::Z80, memory);
        // println!("Running test: {}",test_data.name);
        setup_z80(&mut emulator, &test_data.initial).expect("Failed to setup Z80");
        emulator.step().expect("Failed to step");
        assert_z80(&mut emulator, &test_data.final_state);
    }
}
macro_rules! include_test_data {
    ($test_data_path:literal ) => {{
        use std::fs::read_to_string;
        use std::path::PathBuf;
        let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        full_path.push("tests/z80/v1/");
        full_path.push($test_data_path);
        let test_data_str = &read_to_string(full_path).expect("Failed to read test data");
        // let test_data_str = include_str!(concat!(
        //         env!("CARGO_MANIFEST_DIR"),
        //         "/tests/z80/v1/",
        //         $test_data_path
        //     ));
        let test_data: Vec<TestData> =
            serde_json::from_str(test_data_str).expect("Failed to parse test data");
        test_data
    }};
}

pub(crate) use include_test_data;

macro_rules! test_z80 {
    ($test_data_path:literal) => {
        // use crate::emu_lib::cpu::z80::test::{include_test_data,test_z80_w_data,TestData,TestState};
        paste::item! {
            #[allow(non_snake_case)]
            #[test]
            fn [< test_json >]() {
                let test_data = include_test_data!($test_data_path);
                test_z80_w_data(test_data);
            }
        }
    };
}

pub(crate) use test_z80;
