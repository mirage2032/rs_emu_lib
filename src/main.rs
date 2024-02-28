use emu_lib::emulator::Emulator;

use crate::emu_lib::cpu::BaseInstruction;

mod emu_lib;

fn main() {
    let mut emulator = Emulator::new(emu_lib::cpu::CPUType::Z80);
    emulator.memory.load("roms/rom.z80.bin").expect("Failed to load ROM");
    emulator.run_w_cb(2.0, Some(|_: &mut Emulator, instruction: &Box<dyn BaseInstruction>| {
        println!("{}", instruction);
    }));
}
