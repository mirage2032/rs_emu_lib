use serde::{Deserialize, Serialize};

use crate::cpu::z80::Z80;
use crate::emulator::Emulator;
use crate::memory::memdevices::RAM;
use crate::memory::{Memory, MemoryDevice};

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
    #[serde(rename = "initial")]
    initial_state: TestState,
    #[serde(rename = "final")]
    final_state: TestState,
    cycles: Vec<(u16, Option<u8>, String)>,
}

fn setup_z80(emulator: &mut Emulator<Z80>, state: &TestState) -> Result<(), String> {
    let registers = &mut emulator.cpu.registers;
    registers.pc = state.pc;
    registers.sp = state.sp;
    registers.gp.a = state.a;
    registers.gp.b = state.b;
    registers.gp.c = state.c;
    registers.gp.d = state.d;
    registers.gp.e = state.e;
    registers.gp.f = state.f.into();
    registers.gp.h = state.h;
    registers.gp.l = state.l;
    registers.i = state.i;
    registers.r = state.r;
    //registers.other.insert("ei",BaseRegister::Bit8(state.ei));
    //registers.other.insert("wz",BaseRegister::Bit16(state.wz));
    registers.ix = state.ix;
    registers.iy = state.iy;
    registers.gp_alt.af = state.af_;
    registers.gp_alt.bc = state.bc_;
    registers.gp_alt.de = state.de_;
    registers.gp_alt.hl = state.hl_;
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

fn assert_z80(emulator: &mut Emulator<Z80>, test_state: &TestState) {
    let registers = &emulator.cpu.registers;
    assert_eq!(registers.gp.a, test_state.a);
    assert_eq!(registers.gp.b, test_state.b);
    assert_eq!(registers.gp.c, test_state.c);
    assert_eq!(registers.gp.d, test_state.d);
    assert_eq!(registers.gp.e, test_state.e);
    assert_eq!(registers.gp.f, test_state.f.into());
    assert_eq!(registers.gp.h, test_state.h);
    assert_eq!(registers.gp.l, test_state.l);
    assert_eq!(registers.i, test_state.i);
    assert_eq!(registers.r, test_state.r);
    assert_eq!(registers.gp_alt.af, test_state.af_);
    assert_eq!(registers.gp_alt.bc, test_state.bc_);
    assert_eq!(registers.gp_alt.de, test_state.de_);
    assert_eq!(registers.gp_alt.hl, test_state.hl_);
    assert_eq!(registers.ix, test_state.ix);
    assert_eq!(registers.iy, test_state.iy);
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
        let mut emulator: Emulator<Z80> = Emulator::new_w_mem(memory);
        // println!("Running test: {}",test_data.name);
        setup_z80(&mut emulator, &test_data.initial_state).expect("Failed to setup Z80");
        emulator.step().expect("Failed to step");
        assert_z80(&mut emulator, &test_data.final_state);
        assert_eq!(emulator.cycles, test_data.cycles.len());
    }
}
macro_rules! include_test_data {
    ($test_data_path:expr ) => {{
        // use std::fs::read_to_string;
        // use std::path::PathBuf;
        use emu_lib_json_tests::get_z80_tests;
        // let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // full_path.push("tests/z80/v1/");
        // full_path.push($test_data_path);
        // let test_data_str = &read_to_string(full_path).expect("Failed to read test data");
        let test_data_str = get_z80_tests($test_data_path).unwrap();
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
    // Case for a single parameter
    ($test_data_path:expr) => {
        paste::paste! {
            const TEST_PATH: &str = concat!($test_data_path, ".json");

            #[allow(non_snake_case)]
            #[test]
            fn [< test_json >]() {
                let test_data = include_test_data!(TEST_PATH);
                test_z80_w_data(test_data);
            }
        }
    };
    // Case for multiple parameters
    ($test_data_path_b0:expr,$test_data_path_b1:expr) => {
        paste::paste! {
            // Create a single concatenated path with spaces and append `.json`
            const TEST_PATH: &str = concat!($test_data_path_b0," ", $test_data_path_b1, ".json");

            #[allow(non_snake_case)]
            #[test]
            fn [< test_json >]() {
                let test_data = include_test_data!(TEST_PATH);
                test_z80_w_data(test_data);
            }
        }
    };
}

pub(crate) use test_z80;
