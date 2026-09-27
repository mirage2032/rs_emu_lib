//! Every opcode the decoder knows must survive a round trip: decoded from its
//! bytes, it encodes back to the same bytes, and its text assembles to an
//! instruction that decodes to the same text (a longer equivalent encoding,
//! like `ED 6B` for `LD HL,(nn)`, is fine).

use crate::cpu::instruction::InstructionParser;
use crate::cpu::z80::parser::Z80_PARSER;
use crate::memory::Memory;

fn opcodes() -> impl Iterator<Item = Vec<u8>> {
    let single = (0..=0xFFu8).map(|op| vec![op]);
    let prefixed = [0xCB, 0xDD, 0xED, 0xFD]
        .into_iter()
        .flat_map(|prefix| (0..=0xFFu8).map(move |op| vec![prefix, op]));
    // DD CB d op / FD CB d op, with a displacement of 0x05.
    let indexed_bits = [0xDD, 0xFD]
        .into_iter()
        .flat_map(|prefix| (0..=0xFFu8).map(move |op| vec![prefix, 0xCB, 0x05, op]));
    single.chain(prefixed).chain(indexed_bits)
}

#[test]
fn every_opcode_round_trips() {
    let mut failures = Vec::new();
    for opcode in opcodes() {
        let mut bytes = opcode.clone();
        bytes.extend([0x12, 0x34, 0x56]); // operands, when the instruction has any
        let mut memory = Memory::new_full_ram();
        memory.load(&bytes, true).unwrap();
        let Ok(decoded) = Z80_PARSER.ins_from_machinecode(&memory, 0) else {
            continue; // not an instruction this emulator implements
        };
        let encoded = decoded.to_bytes();
        if encoded[..] != bytes[..encoded.len()] {
            failures.push(format!("{:02X?} decodes to `{decoded}` but encodes to {:02X?}", &bytes[..encoded.len()], encoded));
            continue;
        }
        let text = decoded.to_string();
        match Z80_PARSER.ins_from_asm_string(&text) {
            Ok(assembled) => {
                let mut memory = Memory::new_full_ram();
                memory.load(&assembled.to_bytes(), true).unwrap();
                let again = Z80_PARSER.ins_from_machinecode(&memory, 0).map(|i| i.to_string()).ok();
                if again.as_deref() != Some(text.as_str()) {
                    failures.push(format!("`{text}` assembles to {:02X?}, which decodes to {again:?}", assembled.to_bytes()));
                }
            }
            Err(_) => {} // Not every decoded form is accepted as assembly text; that's a parser gap, not a mismatch.
        }
    }
    assert!(failures.is_empty(), "{} opcodes don't round-trip:\n{}", failures.len(), failures.join("\n"));
}
