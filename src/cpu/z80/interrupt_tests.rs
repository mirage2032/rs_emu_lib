//! Interrupt acceptance and HALT, against the Zilog rules: what each interrupt
//! pushes, where it jumps, what it costs, and how it is reported.

use crate::cpu::instruction::ExecutableInstruction;
use crate::cpu::z80::Z80;
use crate::cpu::Cpu;
use crate::emulator::{Emulator, StopReason};
use crate::io::iodevice::IODevice;
use crate::memory::{Memory, MemoryDevice};

const NO_CALLBACK: Option<fn(&mut Emulator<Z80>, &dyn ExecutableInstruction<Z80>)> = None;

/// An emulator with `program` at 0x0000 and the stack at 0x8000.
fn emulator(program: &[u8]) -> Emulator<Z80> {
    let mut memory = Memory::new_full_ram();
    memory.load(program, true).unwrap();
    let mut emu: Emulator<Z80> = Emulator::new_w_mem(memory);
    emu.cpu.registers.sp = 0x8000;
    emu
}

/// `program`, with an IM 1 handler at 0x0038 that does EI; RETI.
fn with_im1_handler(program: &[u8]) -> Vec<u8> {
    let mut memory = vec![0; 0x3B];
    memory[..program.len()].copy_from_slice(program);
    memory[0x38..0x3B].copy_from_slice(&[0xFB, 0xED, 0x4D]);
    memory
}

/// Steps once, and returns what the step displays as and its T-states.
fn step(emu: &mut Emulator<Z80>) -> (String, u16) {
    let instruction = emu.step().unwrap();
    (instruction.to_string(), instruction.common().cycles)
}

/// The return address on top of the stack.
fn pushed(emu: &Emulator<Z80>) -> u16 {
    emu.memory.read_16(emu.cpu.registers.sp).unwrap()
}

#[test]
fn accepting_an_interrupt_is_a_step_of_its_own() {
    // IM 1; EI; NOP; NOP
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x00, 0x00]));
    for _ in 0..3 {
        step(&mut emu);
    }
    let cycles = emu.cycles;
    emu.request_int(0xFF);
    assert_eq!(step(&mut emu), ("INT IM1 -> 0x0038".to_string(), 13));
    assert_eq!(emu.cpu.registers.pc, 0x0038);
    assert_eq!(emu.cpu.registers.sp, 0x7FFE);
    assert_eq!(pushed(&emu), 0x0004);
    assert!(!emu.cpu.interrupts.iff1 && !emu.cpu.interrupts.iff2);
    assert_eq!(emu.cycles - cycles, 13);
    assert_eq!(emu.instructions, 4);
    // The handler's first instruction runs in the next step.
    assert_eq!(step(&mut emu).0, "EI");
}

#[test]
fn ei_holds_int_off_until_after_the_next_instruction() {
    // IM 1; EI; NOP; NOP
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x00, 0x00]));
    emu.request_int(0xFF); // waits: interrupts are disabled
    step(&mut emu); // IM 1
    step(&mut emu); // EI
    assert_eq!(step(&mut emu).0, "NOP");
    assert_eq!(step(&mut emu).0, "INT IM1 -> 0x0038");
    assert_eq!(pushed(&emu), 0x0004);
}

#[test]
fn ei_does_not_hold_nmi_off() {
    let mut emu = emulator(&[0xFB, 0x00]);
    step(&mut emu); // EI
    emu.request_nmi();
    assert_eq!(step(&mut emu), ("NMI -> 0x0066".to_string(), 11));
    assert_eq!(pushed(&emu), 0x0001);
}

#[test]
fn im2_reads_the_handler_address_from_the_vector_table() {
    // IM 2; EI; NOP
    let mut emu = emulator(&[0xED, 0x5E, 0xFB, 0x00, 0x00]);
    emu.cpu.registers.i = 0x40;
    emu.memory.write_16(0x4010, 0x1234).unwrap();
    for _ in 0..3 {
        step(&mut emu);
    }
    emu.request_int(0x10);
    assert_eq!(step(&mut emu), ("INT IM2 (0x4010) -> 0x1234".to_string(), 19));
    assert_eq!(emu.cpu.registers.pc, 0x1234);
    assert_eq!(pushed(&emu), 0x0004);
}

#[test]
fn the_cpu_interrupt_mode_decides_where_int_goes() {
    // IM n; EI; NOP, then INT with 0xFF on the data bus: RST 38h in IM 0, ignored in
    // IM 1, and the vector at 0x40FF in IM 2.
    for (opcode, im, target, cycles) in [
        (0x46, 0, 0x0038, 13),
        (0x56, 1, 0x0038, 13),
        (0x5E, 2, 0x1234, 19),
    ] {
        let mut emu = emulator(&[0xED, opcode, 0xFB, 0x00, 0x00]);
        emu.cpu.registers.i = 0x40;
        emu.memory.write_16(0x40FF, 0x1234).unwrap();
        for _ in 0..3 {
            step(&mut emu);
        }
        assert_eq!(emu.cpu.interrupts.im, im);
        emu.request_int(0xFF);
        let (_, taken) = step(&mut emu);
        assert_eq!((emu.cpu.registers.pc, taken), (target, cycles), "IM {im}");
        assert_eq!(pushed(&emu), 0x0004, "IM {im}");
    }
}

#[test]
fn nmi_pushes_pc_and_leaves_iff2_for_retn() {
    // EI; NOP; NOP, and RETN at the NMI handler.
    let mut program = vec![0; 0x68];
    program[0] = 0xFB;
    program[0x66..0x68].copy_from_slice(&[0xED, 0x45]);
    let mut emu = emulator(&program);
    step(&mut emu); // EI
    step(&mut emu); // NOP
    emu.request_int(0xFF);
    emu.request_nmi(); // goes first
    assert_eq!(step(&mut emu), ("NMI -> 0x0066".to_string(), 11));
    assert_eq!(emu.cpu.registers.sp, 0x7FFE);
    assert_eq!(pushed(&emu), 0x0002);
    assert!(!emu.cpu.interrupts.iff1);
    assert!(emu.cpu.interrupts.iff2);
    assert_eq!(step(&mut emu).0, "RETN");
    assert_eq!((emu.cpu.registers.pc, emu.cpu.registers.sp), (0x0002, 0x8000));
    assert!(emu.cpu.interrupts.iff1);
    // The INT that waited behind the NMI is taken now (IM 0: RST 38h from the bus).
    assert_eq!(step(&mut emu).0, "INT IM0 RST 0x38 -> 0x0038");
}

#[test]
fn a_nested_nmi_keeps_iff2() {
    let mut emu = emulator(&[0xFB, 0x00]);
    step(&mut emu); // EI
    emu.request_nmi();
    step(&mut emu);
    emu.request_nmi(); // inside the first NMI's handler
    step(&mut emu);
    assert_eq!(emu.cpu.registers.pc, 0x0066);
    assert!(!emu.cpu.interrupts.iff1);
    assert!(emu.cpu.interrupts.iff2, "IFF2 must still hold IFF1 from before the first NMI");
}

#[test]
fn a_halted_cpu_idles_until_an_interrupt_and_resumes_after_the_halt() {
    // IM 1; EI; HALT
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x76]));
    step(&mut emu); // IM 1
    step(&mut emu); // EI
    assert_eq!(step(&mut emu), ("HALT".to_string(), 4));
    assert!(emu.cpu.halted());
    assert_eq!(emu.cpu.registers.pc, 0x0004);
    let r = emu.cpu.registers.r;
    for idle in 1..=3 {
        assert_eq!(step(&mut emu), ("HALT (idle)".to_string(), 4));
        assert_eq!(emu.cpu.registers.pc, 0x0004);
        assert_eq!(emu.cpu.registers.r, r + idle);
    }
    emu.request_int(0xFF);
    assert_eq!(step(&mut emu).0, "INT IM1 -> 0x0038");
    assert!(!emu.cpu.halted());
    assert_eq!(pushed(&emu), 0x0004);
    step(&mut emu); // EI
    assert_eq!(step(&mut emu).0, "RETI");
    assert_eq!(emu.cpu.registers.pc, 0x0004);
    assert!(emu.cpu.interrupts.iff1);
}

#[test]
fn im0_rst_from_the_bus_pushes_pc_unchanged() {
    let mut emu = emulator(&[0xFB, 0x00, 0x00]); // IM 0 after reset
    step(&mut emu); // EI
    step(&mut emu); // NOP
    emu.request_int(0xFF); // RST 38h
    assert_eq!(step(&mut emu), ("INT IM0 RST 0x38 -> 0x0038".to_string(), 13));
    assert_eq!(pushed(&emu), 0x0002);
}

#[test]
fn im0_runs_a_one_byte_instruction_from_the_bus_in_place() {
    let mut emu = emulator(&[0xFB, 0x00, 0x00]);
    emu.cpu.registers.gp.a = 0x41;
    step(&mut emu); // EI
    step(&mut emu); // NOP
    emu.request_int(0x3C); // INC A
    assert_eq!(step(&mut emu), ("INT IM0 INC A -> 0x0002".to_string(), 4 + 2));
    assert_eq!(emu.cpu.registers.gp.a, 0x42);
    assert_eq!((emu.cpu.registers.pc, emu.cpu.registers.sp), (0x0002, 0x8000));
    assert!(!emu.cpu.interrupts.iff1);
}

#[test]
fn im0_rejects_a_byte_that_is_not_a_whole_instruction() {
    let mut emu = emulator(&[0xFB, 0x00, 0x00]);
    step(&mut emu); // EI
    step(&mut emu); // NOP
    emu.request_int(0xCD); // CALL nn: the bus can't supply its address
    let error = emu.step().unwrap_err();
    assert!(error.contains("0xCD"), "{error}");
    assert!(emu.cpu.interrupts.iff1 && emu.cpu.interrupts.iff2);
    assert_eq!((emu.cpu.registers.pc, emu.cpu.registers.sp), (0x0002, 0x8000));
}

#[test]
fn an_interrupt_during_ldir_returns_to_the_ldir() {
    // IM 1; LD BC,0x0010; LDIR, which copies one byte per step.
    let mut emu = emulator(&[0xED, 0x56, 0x01, 0x10, 0x00, 0xED, 0xB0]);
    step(&mut emu); // IM 1
    step(&mut emu); // LD BC,0x0010
    emu.cpu.interrupts.iff1 = true;
    step(&mut emu); // LDIR
    assert_eq!(emu.cpu.registers.pc, 0x0005);
    emu.request_int(0xFF);
    assert_eq!(step(&mut emu).0, "INT IM1 -> 0x0038");
    assert_eq!(pushed(&emu), 0x0005);
    assert_eq!(emu.cpu.registers.gp.bc, 0x000F);
}

#[test]
fn save_states_keep_the_interrupt_state() {
    // IM 2; EI; HALT
    let mut emu = emulator(&[0xED, 0x5E, 0xFB, 0x76]);
    for _ in 0..3 {
        step(&mut emu);
    }
    emu.request_nmi();
    emu.request_int(0x10);
    emu.set_int_line(true, 0x20);
    let mut restored = emulator(&[0x00]);
    restored.load(emu.save().unwrap(), false, true).unwrap();
    assert_eq!(restored.cpu.interrupts, emu.cpu.interrupts);
    assert!(restored.cpu.halted());
    let interrupts = restored.cpu.interrupts;
    assert_eq!(interrupts.im, 2);
    assert!(interrupts.iff1 && interrupts.iff2 && interrupts.nmi_pending);
    assert_eq!((interrupts.int_request, interrupts.int_line), (Some(0x10), Some(0x20)));
}

#[test]
fn a_held_int_line_fires_again_until_it_is_released() {
    // IM 1; EI; NOP; NOP; NOP
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x00, 0x00, 0x00]));
    step(&mut emu); // IM 1
    step(&mut emu); // EI
    emu.set_int_line(true, 0xFF);
    assert_eq!(step(&mut emu).0, "NOP");
    for _ in 0..2 {
        assert_eq!(step(&mut emu).0, "INT IM1 -> 0x0038");
        assert_eq!(step(&mut emu).0, "EI");
        assert_eq!(step(&mut emu).0, "RETI");
    }
    emu.set_int_line(false, 0);
    assert_eq!(step(&mut emu).0, "NOP");
}

#[test]
fn a_requested_int_fires_once() {
    // IM 1; EI; NOP; NOP; NOP
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x00, 0x00, 0x00]));
    step(&mut emu); // IM 1
    step(&mut emu); // EI
    emu.request_int(0xFF);
    emu.request_int(0xFF); // merges with the first
    assert_eq!(step(&mut emu).0, "NOP");
    assert_eq!(step(&mut emu).0, "INT IM1 -> 0x0038");
    assert_eq!(step(&mut emu).0, "EI");
    assert_eq!(step(&mut emu).0, "RETI");
    assert_eq!(step(&mut emu).0, "NOP");
}

#[test]
fn run_ticks_stops_on_halt_only_when_nothing_but_an_nmi_can_wake_the_cpu() {
    // DI; HALT: halted with interrupts disabled.
    let mut emu = emulator(&[0xF3, 0x76]);
    assert!(matches!(emu.run_ticks(1000.0, &NO_CALLBACK), Err(StopReason::Halt)));
    assert_eq!((emu.cycles, emu.cpu.registers.pc), (8, 0x0002));
    emu.request_nmi();
    emu.run_ticks(11.0, &NO_CALLBACK).unwrap();
    assert!(!emu.cpu.halted());
    assert_eq!(emu.cpu.registers.pc, 0x0066);

    // IM 1; EI; HALT: an INT could wake it, so it keeps idling.
    let mut emu = emulator(&[0xED, 0x56, 0xFB, 0x76]);
    assert!(emu.run_ticks(1000.0, &NO_CALLBACK).unwrap() >= 1000.0);
    assert!(emu.cpu.halted());
    emu.request_int(0xFF);
    emu.run_ticks(13.0, &NO_CALLBACK).unwrap();
    assert!(!emu.cpu.halted());
    assert_eq!(emu.cpu.registers.pc, 0x0038);
}

#[test]
fn a_breakpoint_on_the_handler_stops_before_its_first_instruction() {
    // IM 1; EI; NOP; NOP
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x00, 0x00]));
    emu.breakpoints.push(0x0038);
    for _ in 0..3 {
        step(&mut emu);
    }
    emu.request_int(0xFF);
    assert!(matches!(emu.run_ticks(100.0, &NO_CALLBACK), Err(StopReason::Breakpoint)));
    assert_eq!(emu.cpu.registers.pc, 0x0038);
    assert_eq!(emu.instructions, 4); // the acceptance is the only step run
}

#[test]
fn a_breakpoint_after_a_halt_fires_when_the_cpu_returns_there() {
    // IM 1; EI; HALT, with a breakpoint on the byte after the HALT.
    let mut emu = emulator(&with_im1_handler(&[0xED, 0x56, 0xFB, 0x76]));
    emu.breakpoints.push(0x0004);
    assert!(emu.run_ticks(100.0, &NO_CALLBACK).is_ok()); // idling doesn't stop at it
    emu.request_int(0xFF);
    assert!(matches!(emu.run_ticks(100.0, &NO_CALLBACK), Err(StopReason::Breakpoint)));
    assert_eq!(emu.cpu.registers.pc, 0x0004);
    assert!(!emu.cpu.halted());
}

/// A device that holds INT until it is acknowledged, and supplies an IM 2 vector.
struct VectoredDevice {
    holding_int: bool,
    vector: u8,
}

impl IODevice for VectoredDevice {
    fn ports(&self) -> Vec<u8> {
        vec![]
    }
    fn read(&self, _: u8) -> Result<u8, &'static str> {
        Ok(0)
    }
    fn write(&mut self, _: u8, _: u8) -> Result<(), &'static str> {
        Ok(())
    }
    fn int_pending(&self) -> bool {
        self.holding_int
    }
    fn int_ack(&mut self) -> u8 {
        self.holding_int = false;
        self.vector
    }
}

#[test]
fn a_device_can_hold_int_and_supply_the_im2_vector() {
    // IM 2; EI; NOP
    let mut emu = emulator(&[0xED, 0x5E, 0xFB, 0x00, 0x00]);
    emu.cpu.registers.i = 0x40;
    emu.memory.write_16(0x4020, 0x1234).unwrap();
    let device = VectoredDevice { holding_int: true, vector: 0x20 };
    emu.io.add_device(Box::new(device)).unwrap();
    assert!(emu.io.int_pending());
    for _ in 0..3 {
        step(&mut emu);
    }
    assert_eq!(step(&mut emu).0, "INT IM2 (0x4020) -> 0x1234");
    assert!(!emu.io.int_pending(), "the device was acknowledged");
}
