//! The block instructions: LDI, CPI, INI and OUTI, and their relatives. Bit 3 of
//! the opcode makes HL step down instead of up (LDD, CPD, IND, OUTD), and bit 4
//! repeats the instruction until it's done (LDIR, CPIR, INIR, OTIR, and the ones
//! that step down). Each repetition is a step of its own, which runs the
//! instruction again from its own address, so interrupts can be taken between
//! them, as on the Z80. Each family is one type, generic over the two bits.

mod cp;
mod inp;
mod ld;
mod out;

pub use cp::{CPD, CPDR, CPI, CPIR, CP_BLOCK};
pub use inp::{IND, INDR, INI, INIR, IN_BLOCK};
pub use ld::{LDD, LDDR, LDI, LDIR, LD_BLOCK};
pub use out::{OTDR, OTIR, OUTD, OUTI, OUT_BLOCK};

use crate::cpu::instruction::InstructionCommon;

/// The byte after ED: `base` with bit 3 set to step down and bit 4 to repeat.
const fn opcode(base: u8, down: bool, repeat: bool) -> u8 {
    base | (down as u8) << 3 | (repeat as u8) << 4
}

/// The variant's mnemonic: one of `names`, which are in the order I, D, IR, DR.
fn name(names: [&'static str; 4], down: bool, repeat: bool) -> &'static str {
    names[down as usize | (repeat as usize) << 1]
}

/// `address` stepped down or up by one.
fn step(address: u16, down: bool) -> u16 {
    if down {
        address.wrapping_sub(1)
    } else {
        address.wrapping_add(1)
    }
}

fn common() -> InstructionCommon {
    InstructionCommon::new(2, 16, true)
}

/// Runs the instruction again (PC stays on it, 21 T-states) or finishes it (16).
fn repeat(common: &mut InstructionCommon, again: bool) {
    (common.cycles, common.increment_pc) = if again { (21, false) } else { (16, true) };
}
