#![feature(slice_index_methods)]

use emu_lib::emulator::Emulator;

use crate::emu_lib::cpu::{BaseInstruction, Cpu, InstructionDecoder, RegisterOps, SingleRegister, z80};
use crate::emu_lib::memory::{MemoryError, ReadableMemory};

mod emu_lib;

fn print_registers(registers: &dyn RegisterOps) {
    let register_map = registers.get_all();
    for i in ["af", "bc", "de", "hl", "ix", "iy"].iter() {
        match register_map.get(i).unwrap() {
            SingleRegister::Bit8(v) => { print!("{} {:02X}, ", i, v); }
            SingleRegister::Bit16(v) => { print!("{} {:04X}, ", i, v); }
        }
    };
    print!("pc {:04X}, ", registers.pc());
    print!("sp {:04X}", registers.sp().last().unwrap_or(&0));
    println!();
}

fn main() {
    let mut emulator = Emulator::new(emu_lib::cpu::CPUType::Z80);
    emulator.memory.load("roms/rom.z80.bin");
    print_registers(emulator.cpu.registers());
    emulator.run_w_cb(2.0, Some(|emu: &mut Emulator, instruction: &dyn BaseInstruction| {
        println!("{}", instruction);
        print_registers(emu.cpu.registers());
    }
    ));
    let mem = emulator.memory.read_8(0xAABB);
    println!("Mem 0xAABB: {:02X}", mem.unwrap());
    let instr = z80::Z80::decode(&vec![0x01, 0x41, 0x80], 0).unwrap();
    println!("{}", instr);
}
