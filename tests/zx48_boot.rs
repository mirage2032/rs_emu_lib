//! Boots the ZX Spectrum 48K ROM (the one emu_cli runs) with its 50 Hz frame
//! interrupt, INT held for the first 32 T-states of every 69,888 T-state frame as
//! emu_cli does, and checks that the ROM's interrupt handler runs exactly once per
//! frame.

use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::memory::{Memory, MemoryDevice};

/// T-states per frame on a 48K Spectrum.
const FRAME_TSTATES: usize = 69_888;
/// T-states the ULA holds INT active for, at the start of each frame.
const INT_TSTATES: usize = 32;
/// FRAMES, the ROM's frame counter, which its IM 1 handler increments.
const FRAMES: u16 = 0x5C78;

#[test]
fn zx48_rom_boots_and_takes_one_interrupt_per_frame() {
    let rom = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/emu_cli/roms/zx48.rom"));
    let mut memory = Memory::new_full_ram();
    memory.load(rom, true).unwrap();
    let mut emu: Emulator<Z80> = Emulator::new_w_mem(memory);
    let frames = |emu: &Emulator<Z80>| emu.memory.read_16(FRAMES).unwrap();

    // The ROM enables interrupts in IM 1 after about 82 frames of memory checks.
    let mut frames_at_140 = 0;
    for frame in 1..=150 {
        while emu.cycles < frame * FRAME_TSTATES {
            if let Err(error) = emu.step() {
                panic!("frame {frame}, PC 0x{:04X}: {error}", emu.cpu.registers.pc);
            }
            emu.set_int_line(emu.cycles % FRAME_TSTATES < INT_TSTATES, 0xFF);
        }
        if frame == 140 {
            frames_at_140 = frames(&emu);
        }
    }

    assert_eq!(emu.cpu.interrupts.im, 1);
    assert!(emu.cpu.interrupts.iff1);
    assert!(frames(&emu) >= 20, "FRAMES = {}", frames(&emu));
    assert_eq!(frames(&emu) - frames_at_140, 10, "one interrupt per frame");
}
