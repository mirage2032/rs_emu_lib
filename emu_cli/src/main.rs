use std::path::PathBuf;
use emu_lib::cpu::Cpu;

use emu_lib::cpu::{
    instruction::BaseInstruction,
    registers::{AllRegisters, GPByteRegisters},
};
use emu_lib::cpu::instruction::ExecutableInstruction;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::{errors::MemoryError, memdevices::RAM, Memory, MemoryDevice};

mod memdsp;
use memdsp::MemViz;

fn print_registers(registers: &AllRegisters) {
    println!("PC: {:04X}, SP: {:04X}", registers.pc, registers.sp);
    fn print_gp(gp: &GPByteRegisters, suffix: &str) {
        println!(
            "AF{suffix}: {:04X}, BC{suffix}: {:04X}, DE{suffix}: {:04X}, HL{suffix}: {:04X}",
            gp.af, gp.bc, gp.de, gp.hl
        );
    }
    for (i, gp_regs) in registers.gp.iter().enumerate() {
        print_gp(gp_regs, &String::from("'").repeat(i));
    }
    for (key, value) in &registers.other {
        print!("{}: {}, ", key.to_uppercase(), value);
    }
    println!();
}

fn main() {
    let dsp = MemViz::new(64 * 64, 64, 10.0);
    // dsp.randomize();
    // thread::sleep(Duration::from_secs(2));
    println!("Creating emulator");
    let mut memory = Memory::new();
    memory.add_device(Box::new(RAM::new(0x1000)));
    memory.add_device(Box::new(dsp));
    memory.add_device(Box::new(RAM::new(0x10000 - 64 * 64 - 0x1000)));
    let mut emulator: Emulator<Z80> = Emulator::new_w_mem(memory);
    let rom_path: PathBuf = PathBuf::from("roms/color.bin");
    println!("Loading rom: {}", rom_path.to_str().unwrap());
    match emulator.memory.load_file(&rom_path) {
        Ok(_) => {}
        Err(e) => {
            for err in e {
                if let MemoryError::File(e) = err {
                    panic!("{}", e)
                }
            }
        }
    };
    println!("Running emulator");
    // print_registers(emulator.cpu.registers());
    let freq = 600_000.0;
    let stop_reason = emulator.run_with_callback(
        freq,
        Some(move |emu: &mut Emulator<_>, instruction: &dyn ExecutableInstruction<_>| {
            // println!("{}", instruction);
            //
            // print_registers(emu.cpu.registers());
        }),
        // 1
        freq as usize / 60,
    );
    println!("Emulator stopped");
    match stop_reason {
        emu_lib::emulator::StopReason::Breakpoint => println!("Breakpoint"),
        emu_lib::emulator::StopReason::Halt => println!("Halted"),
        emu_lib::emulator::StopReason::Error(e) => {
            let pc = emulator.cpu.registers().pc;
            let instruction = emulator
                .cpu
                .parser()
                .ins_from_mem(&emulator.memory, pc)
                .expect("Error decoding instruction");
            println!("Error: {} while executing \"{}\"", e, instruction)
        }
    }
}
