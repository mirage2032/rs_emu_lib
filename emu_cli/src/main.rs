use emu_lib::cpu::{BaseInstruction, RegisterOps, SingleRegister};
use emu_lib::emulator::Emulator;

fn print_registers(registers: &dyn RegisterOps) {
    let register_map = registers.get_all();
    print!("Registers: ");
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
    let _ = emulator.memory.load("roms/rom.z80.bin");
    print_registers(emulator.cpu.registers());
    let err = emulator.run_w_cb(2.0, Some(|emu: &mut Emulator, instruction: &dyn BaseInstruction| {
        println!("{}", instruction);
        print_registers(emu.cpu.registers());
    }
    ));
    match err {
        emu_lib::emulator::StopReason::Breakpoint => println!("Breakpoint"),
        emu_lib::emulator::StopReason::Halt => println!("Halted"),
        emu_lib::emulator::StopReason::Error(e) => {
            let pc = *emulator.cpu.registers().pc();
            let instruction = emulator.cpu.decode_mem(&emulator.memory, pc).unwrap();
            println!("Error: {} while executing \"{}\"", e, instruction)
        }
    }
}
