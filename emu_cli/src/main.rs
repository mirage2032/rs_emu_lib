use std::thread;
use std::time::Duration;

use emu_lib::cpu::{BaseInstruction, RegisterOps, SingleRegister};
use emu_lib::emulator::Emulator;
use emu_lib::memory::{Memory, MemoryError, RAM};
use memdsp::MemViz;

mod memdsp;

fn print_registers(registers: &dyn RegisterOps) {
    let register_map = registers.get_all();
    print!("Registers: ");
    for i in ["af", "bc", "de", "hl", "ix", "iy", "sp", "pc"].iter() {
        match register_map.get(i).unwrap() {
            SingleRegister::Bit8(v) => { print!("{} {:02X}, ", i, v); }
            SingleRegister::Bit16(v) => { print!("{} {:04X}, ", i, v); }
        }
    };
    print!("pc {:04X}, ", registers.pc());
    println!();
}

fn main() {
    let mut dsp = MemViz::new(64 * 64, 64);
    dsp.start_thread(12.0);
    dsp.randomize();
    thread::sleep(Duration::from_millis(1000));
    println!("Creating emulator");
    let mut memory = Memory::new();
    let bank = RAM::new(0x2000);
    memory.add_device(Box::new(bank));
    memory.add_device(Box::new(dsp));
    let mut emulator = Emulator::new_w_mem(emu_lib::cpu::CPUType::Z80, memory);
    let rom_path = "roms/rom.z80.bin".to_string();
    println!("Loading rom: {}", rom_path);
    match emulator.memory.load(rom_path) {
        Ok(_) => {}
        Err(e) => {
            for err in e {
                match err {
                    MemoryError::FileError | MemoryError::ReadError => {
                        panic!("Can't open or read the rom file");
                    }
                    MemoryError::EndOfMem(location) => {
                        println!("Mapped memory ends at {}, skipping", location);
                    }
                    MemoryError::ReadOnly(location, size) => {
                        println!("Memory between {} and {} is read-only, skipping", location, location + size);
                    }
                }
            }
        }
    };
    println!("Running emulator");
    print_registers(emulator.cpu.registers());
    let err = emulator.run_w_cb(10.0, Some(|emu: &mut Emulator, instruction: &dyn BaseInstruction| {
        println!("{}", instruction);
        print_registers(emu.cpu.registers());
    }
    ));
    println!("Emulator stopped");
    match err {
        emu_lib::emulator::StopReason::Breakpoint => println!("Breakpoint"),
        emu_lib::emulator::StopReason::Halt => println!("Halted"),
        emu_lib::emulator::StopReason::Error(e) => {
            let pc = *emulator.cpu.registers().pc();
            let instruction = emulator.cpu.parser().ins_from_mem(&emulator.memory, pc).expect("Error decoding instruction");
            println!("Error: {} while executing \"{}\"", e, instruction)
        }
    }
    std::thread::sleep(Duration::from_millis(10000));
}
