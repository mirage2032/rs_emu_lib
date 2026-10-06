//! Runs the SingleStepTests Z80 suite (<https://github.com/SingleStepTests/z80>).
//! Each file of the suite is one test, named by the opcode it covers (`ed b0`,
//! `dd cb __ 06`), and holds 1,000 cases. A case sets the registers, memory and
//! ports, runs one instruction, and compares the result with the real Z80's:
//! registers, flags, interrupt state, memory, which addresses were read and
//! written, port accesses and T-states. (WZ, P and Q, internal state the emulator
//! doesn't model, aren't compared.) It also checks that the instruction decoded is
//! the one the file covers and that it encodes back to its bytes.
//!
//! The data comes from the `emu_lib_json_tests` dev-dependency and is read when the
//! tests run. Files for opcodes the emulator doesn't implement are ignored.
//!
//! ```text
//! cargo test --test singlestep                     # every file
//! cargo test --test singlestep -- "ed b0"          # files whose name contains `ed b0`
//! cargo test --test singlestep -- --list --ignored # opcodes not implemented yet
//! ```

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use emu_lib::cpu::instruction::InstructionParser;
use emu_lib::cpu::z80::parser::Z80_PARSER;
use emu_lib::cpu::z80::Z80;
use emu_lib::emulator::Emulator;
use emu_lib::io::iodevice::IODevice;
use emu_lib::io::IO;
use emu_lib::memory::{Memory, MemoryDevice};
use libtest_mimic::{Arguments, Failed, Trial};
use serde::Deserialize;

/// Files whose instructions take F's bits 3 and 5 (undocumented copies of other
/// bits) from internal registers the emulator doesn't model. Only those two bits
/// of F go unchecked for them.
#[rustfmt::skip]
const UNCHECKED_F_BITS_3_5: &[&str] = &[
    "37", "3f", // SCF, CCF: from Q, which records whether the last instruction set F
    // BIT b,(HL): from WZ, the address latch
    "cb 46", "cb 4e", "cb 56", "cb 5e", "cb 66", "cb 6e", "cb 76", "cb 7e",
];

/// How many failing cases a failed test lists.
const LISTED_FAILURES: usize = 5;

#[derive(Deserialize)]
struct State {
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
    ix: u16,
    iy: u16,
    af_: u16,
    bc_: u16,
    de_: u16,
    hl_: u16,
    ei: u8,
    im: u8,
    iff1: u8,
    iff2: u8,
    ram: Vec<(u16, u8)>,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    initial: State,
    #[serde(rename = "final")]
    expected: State,
    /// The bus at each T-state: address, data, and which of the RD, WR, MREQ and
    /// IORQ pins are active (`r-m-` is a memory read, `-wm-` a memory write).
    cycles: Vec<(Option<u16>, Option<u8>, String)>,
    /// Port accesses in order: address, data, and `r` or `w`.
    #[serde(default)]
    ports: Vec<(u16, u8, char)>,
}

thread_local! {
    /// The memory addresses read, in order. Each test runs on one thread.
    static READS: RefCell<Vec<u16>> = const { RefCell::new(Vec::new()) };
}

fn record_read(address: u16, _data: u8) {
    READS.with(|reads| reads.borrow_mut().push(address));
}

/// Port reads get the values the case lists, in order. Every access is logged.
#[derive(Default)]
struct PortScript {
    reads: VecDeque<u8>,
    log: Vec<(u8, u8, char)>,
}

struct ScriptedPorts(Arc<Mutex<PortScript>>);

impl IODevice for ScriptedPorts {
    fn ports(&self) -> Vec<u8> {
        (0..=0xFF).collect()
    }
    fn read(&self, port: u8) -> Result<u8, &'static str> {
        let mut script = self.0.lock().unwrap();
        let data = script.reads.pop_front().unwrap_or(0xFF);
        script.log.push((port, data, 'r'));
        Ok(data)
    }
    fn write(&mut self, port: u8, data: u8) -> Result<(), &'static str> {
        self.0.lock().unwrap().log.push((port, data, 'w'));
        Ok(())
    }
}

/// The opcode a file is named by, with `None` for the displacement byte (`__`).
/// `None` for a file that isn't named by an opcode (the suite's `100` and `101`).
fn opcode_pattern(name: &str) -> Option<Vec<Option<u8>>> {
    name.split(' ')
        .map(|byte| match byte {
            "__" => Some(None),
            _ if byte.len() == 2 => u8::from_str_radix(byte, 16).ok().map(Some),
            _ => None,
        })
        .collect()
}

/// Whether the emulator decodes the opcode.
fn implemented(opcode: &[Option<u8>]) -> bool {
    let mut bytes: Vec<u8> = opcode.iter().map(|byte| byte.unwrap_or(0x05)).collect();
    bytes.extend([0x12, 0x34, 0x56]); // operands, when the instruction has any
    Z80_PARSER.ins_from_machinecode(&bytes, 0).is_ok()
}

fn set_state(emu: &mut Emulator<Z80>, state: &State) {
    let registers = &mut emu.cpu.registers;
    registers.pc = state.pc;
    registers.sp = state.sp;
    registers.gp.a = state.a;
    registers.gp.f = state.f.into();
    registers.gp.b = state.b;
    registers.gp.c = state.c;
    registers.gp.d = state.d;
    registers.gp.e = state.e;
    registers.gp.h = state.h;
    registers.gp.l = state.l;
    registers.i = state.i;
    registers.r = state.r;
    registers.ix = state.ix;
    registers.iy = state.iy;
    registers.gp_alt.af = state.af_;
    registers.gp_alt.bc = state.bc_;
    registers.gp_alt.de = state.de_;
    registers.gp_alt.hl = state.hl_;
    let interrupts = &mut emu.cpu.interrupts;
    interrupts.iff1 = state.iff1 != 0;
    interrupts.iff2 = state.iff2 != 0;
    interrupts.im = state.im;
    interrupts.ei_delay = state.ei != 0;
    for &(address, data) in &state.ram {
        emu.memory.write_8(address, data).unwrap();
    }
}

/// Runs a case and returns what differs from the real Z80. `emu` has zeroed
/// memory and `ports` as its only IO device.
fn run_case(
    emu: &mut Emulator<Z80>,
    ports: &Mutex<PortScript>,
    case: &Case,
    opcode: &[Option<u8>],
    f_mask: u8,
) -> Vec<String> {
    emu.cpu = Z80::default();
    emu.cycles = 0;
    *ports.lock().unwrap() = PortScript {
        reads: case
            .ports
            .iter()
            .filter(|access| access.2 == 'r')
            .map(|access| access.1)
            .collect(),
        log: Vec::new(),
    };
    set_state(emu, &case.initial);

    READS.with(|reads| reads.borrow_mut().clear());
    let instruction = match emu.step() {
        Ok(instruction) => instruction,
        Err(error) => return vec![format!("didn't run: {error}")],
    };
    let reads = READS.with(|reads| reads.take());
    let mut differences = Vec::new();

    // The instruction is the file's, and encodes back to the bytes it was decoded from.
    let bytes = instruction.to_bytes();
    let is_the_files = bytes.len() >= opcode.len()
        && opcode
            .iter()
            .zip(&bytes)
            .all(|(want, &got)| want.map_or(true, |want| want == got));
    let encoded: Vec<String> = bytes.iter().map(|byte| format!("{byte:02X}")).collect();
    let in_memory: Vec<String> = (0..bytes.len() as u16)
        .map(|offset| {
            let address = case.initial.pc.wrapping_add(offset);
            match case.initial.ram.iter().find(|entry| entry.0 == address) {
                Some(&(_, data)) => format!("{data:02X}"),
                None => "--".to_owned(), // the case doesn't give this byte
            }
        })
        .collect();
    if !is_the_files {
        differences.push(format!(
            "decoded as `{instruction}` ({})",
            encoded.join(" ")
        ));
    } else if encoded != in_memory {
        differences.push(format!(
            "`{instruction}` encodes to {}, decoded from {}",
            encoded.join(" "),
            in_memory.join(" ")
        ));
    }
    if instruction.common().length as usize != bytes.len() {
        differences.push(format!(
            "`{instruction}` has length {}, encodes to {} bytes",
            instruction.common().length,
            bytes.len()
        ));
    }

    let want = &case.expected;
    let registers = &emu.cpu.registers;
    let interrupts = &emu.cpu.interrupts;
    let f = u8::from(registers.gp.f);
    if f & f_mask != want.f & f_mask {
        differences.push(format!("F {f:08b}, expected {:08b} (SZ5H3PNC)", want.f));
    }
    for (name, got, want) in [
        ("PC", registers.pc, want.pc),
        ("SP", registers.sp, want.sp),
        ("A", registers.gp.a.into(), want.a.into()),
        ("B", registers.gp.b.into(), want.b.into()),
        ("C", registers.gp.c.into(), want.c.into()),
        ("D", registers.gp.d.into(), want.d.into()),
        ("E", registers.gp.e.into(), want.e.into()),
        ("H", registers.gp.h.into(), want.h.into()),
        ("L", registers.gp.l.into(), want.l.into()),
        ("I", registers.i.into(), want.i.into()),
        ("R", registers.r.into(), want.r.into()),
        ("IX", registers.ix, want.ix),
        ("IY", registers.iy, want.iy),
        ("AF'", registers.gp_alt.af, want.af_),
        ("BC'", registers.gp_alt.bc, want.bc_),
        ("DE'", registers.gp_alt.de, want.de_),
        ("HL'", registers.gp_alt.hl, want.hl_),
        ("IFF1", interrupts.iff1.into(), want.iff1.into()),
        ("IFF2", interrupts.iff2.into(), want.iff2.into()),
        ("IM", interrupts.im.into(), want.im.into()),
        ("EI delay", interrupts.ei_delay.into(), want.ei.into()),
    ] {
        if got != want {
            differences.push(format!("{name} {got:#X}, expected {want:#X}"));
        }
    }

    for &(address, data) in &want.ram {
        let got = emu.memory.read_8(address).unwrap();
        if got != data {
            differences.push(format!("({address:#06X}) {got:#04X}, expected {data:#04X}"));
        }
    }
    // Reads of the instruction's own bytes aren't compared: the decoder reads
    // them its own way.
    let in_instruction = |address: u16| address.wrapping_sub(case.initial.pc) < bytes.len() as u16;
    let mut read: Vec<u16> = reads
        .into_iter()
        .filter(|&address| !in_instruction(address))
        .collect();
    read.sort_unstable();
    let mut want_read: Vec<u16> = case
        .cycles
        .iter()
        .filter(|cycle| cycle.2 == "r-m-")
        .filter_map(|cycle| cycle.0)
        .filter(|&address| !in_instruction(address))
        .collect();
    want_read.sort_unstable();
    if read != want_read {
        differences.push(format!("read {read:04X?}, expected {want_read:04X?}"));
    }
    let mut written = emu.memory.get_changes().clone().unwrap_or_default();
    written.sort_unstable();
    let mut want_written: Vec<u16> = case
        .cycles
        .iter()
        .filter(|cycle| cycle.2 == "-wm-")
        .filter_map(|cycle| cycle.0)
        .collect();
    want_written.sort_unstable();
    if written != want_written {
        differences.push(format!(
            "wrote {written:04X?}, expected {want_written:04X?}"
        ));
    }

    // Ports are 8-bit for now: compare the low byte of the address.
    let port_accesses = &ports.lock().unwrap().log;
    let want_port_accesses: Vec<(u8, u8, char)> = case
        .ports
        .iter()
        .map(|&(address, data, rw)| (address as u8, data, rw))
        .collect();
    if *port_accesses != want_port_accesses {
        differences.push(format!(
            "port accesses {port_accesses:02X?}, expected {want_port_accesses:02X?}"
        ));
    }

    if emu.cycles != case.cycles.len() {
        differences.push(format!(
            "{} T-states, expected {}",
            emu.cycles,
            case.cycles.len()
        ));
    }
    differences
}

/// Runs every case in a file. Fails if any case does, listing the first few.
fn run_file(path: &Path, opcode: &[Option<u8>], f_mask: u8) -> Result<(), Failed> {
    let json = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let cases: Vec<Case> =
        serde_json::from_str(&json).map_err(|error| format!("{}: {error}", path.display()))?;
    // One emulator runs every case: a new one for each would take most of the time.
    let ports = Arc::new(Mutex::new(PortScript::default()));
    let mut emu: Emulator<Z80> = Emulator::new_w_mem(Memory::new_full_ram());
    emu.io = IO::new();
    emu.io
        .add_device(Box::new(ScriptedPorts(ports.clone())))
        .unwrap();
    emu.memory.record_changes(true); // each step starts a new record
    emu.memory.add_read_callback(Some(record_read));
    let mut failed = 0;
    let mut listed = Vec::new();
    for case in &cases {
        let differences = run_case(&mut emu, &ports, case, opcode, f_mask);
        // Zero the memory the case set or wrote, for the next case.
        let written = emu.memory.get_changes().clone().unwrap_or_default();
        for address in case
            .initial
            .ram
            .iter()
            .map(|&(address, _)| address)
            .chain(written)
        {
            emu.memory.write_8(address, 0).unwrap();
        }
        if !differences.is_empty() {
            failed += 1;
            if listed.len() < LISTED_FAILURES {
                listed.push(format!("  {}: {}", case.name, differences.join(", ")));
            }
        }
    }
    if failed == 0 {
        Ok(())
    } else {
        Err(format!(
            "{failed} of {} cases failed:\n{}",
            cases.len(),
            listed.join("\n")
        )
        .into())
    }
}

fn main() {
    let args = Arguments::from_args();
    let dir = Path::new(emu_lib_json_tests::Z80_V1_DIR);
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("can't read the test data in {}: {error}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    files.sort();
    let trials = files
        .into_iter()
        .filter_map(|path| {
            let name = path.file_stem()?.to_str()?.to_owned();
            let opcode = opcode_pattern(&name)?;
            let f_mask = if UNCHECKED_F_BITS_3_5.contains(&name.as_str()) {
                0b1101_0111
            } else {
                0xFF
            };
            let ignored = !implemented(&opcode);
            Some(
                Trial::test(name, move || run_file(&path, &opcode, f_mask))
                    .with_ignored_flag(ignored),
            )
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}
