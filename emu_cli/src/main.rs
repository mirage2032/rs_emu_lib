use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use emu_lib::cpu::instruction::BaseInstruction;
use emu_lib::emulator::Emulator;
use emu_lib::memory::errors::MemoryError;
use emu_lib::memory::memdevices::RAM;
use emu_lib::memory::Memory;
use memdsp::MemViz;

mod memdsp;

fn print_registers(registers: &emu_lib::cpu::registers::AllRegisters) {
    println!("PC: {:04X}, SP: {:04X}", registers.pc, registers.sp);
    fn print_gp(gp: &emu_lib::cpu::registers::GPByteRegisters, suffix: &str) {
        println!(
            "AF{suffix}: {:04X}, BC{suffix}: {:04X}, DE{suffix}: {:04X}, HL{suffix}: {:04X}",
            gp.af, gp.bc, gp.de, gp.hl
        );
    }
    print_gp(&registers.gp[0], "");
    print_gp(&registers.gp[0], "'");
    for (key, value) in &registers.other {
        print!("{}: {}, ", key.to_uppercase(), value);
    }
    println!();
}

fn main() {
    let mut dsp = MemViz::new(64 * 64, 64, 10.0);
    dsp.randomize();
    thread::sleep(Duration::from_secs(2));
    println!("Creating emulator");
    let mut memory = Memory::new();
    let bank = RAM::new(0x2000);
    memory.add_device(Box::new(dsp));
    memory.add_device(Box::new(bank));
    let mut emulator = Emulator::new_w_mem(emu_lib::cpu::CPUType::Z80, memory);
    let rom_path: PathBuf = PathBuf::from("roms/rom.z80.bin");
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
    print_registers(emulator.cpu.registers());
    let err = emulator.run_w_cb(
        20.0,
        Some(|emu: &mut Emulator, instruction: &dyn BaseInstruction| {
            println!("{}", instruction);
            print_registers(emu.cpu.registers());
        }),
    );
    println!("Emulator stopped");
    match err {
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
