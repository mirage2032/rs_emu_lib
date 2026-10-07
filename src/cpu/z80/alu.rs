//! The Z80's arithmetic and logic. Each function returns its result and sets the
//! flags the instruction sets, including bits 5 and 3: undocumented copies of
//! bits of the result (or, for some instructions, of something else). Flags an
//! instruction doesn't change are left alone.

use crate::cpu::registers::Flags;

/// Whether `value` has an even number of bits set: P/V after a logical operation.
pub fn parity(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// Sets S, Z, 5 and 3 from `value`, P/V to its parity, and clears H and N, as
/// logical operations, rotates and `IN r,(C)` do.
pub fn sz53p(f: &mut Flags, value: u8) {
    f.set_sign(value & 0x80 != 0);
    f.set_zero(value == 0);
    f.set_bit5(value & 0x20 != 0);
    f.set_half_carry(false);
    f.set_bit3(value & 0x08 != 0);
    f.set_parity_overflow(parity(value));
    f.set_add_sub(false);
}

/// ADD (`carry` false) and ADC.
pub fn add8(f: &mut Flags, a: u8, b: u8, carry: bool) -> u8 {
    let sum = a as u16 + b as u16 + carry as u16;
    let result = sum as u8;
    f.set_sign(result & 0x80 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry((a & 0x0F) + (b & 0x0F) + carry as u8 > 0x0F);
    f.set_bit3(result & 0x08 != 0);
    f.set_parity_overflow((a ^ result) & (b ^ result) & 0x80 != 0);
    f.set_add_sub(false);
    f.set_carry(sum > 0xFF);
    result
}

/// SUB (`carry` false) and SBC.
pub fn sub8(f: &mut Flags, a: u8, b: u8, carry: bool) -> u8 {
    let result = a.wrapping_sub(b).wrapping_sub(carry as u8);
    f.set_sign(result & 0x80 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry((a & 0x0F) < (b & 0x0F) + carry as u8);
    f.set_bit3(result & 0x08 != 0);
    f.set_parity_overflow((a ^ b) & (a ^ result) & 0x80 != 0);
    f.set_add_sub(true);
    f.set_carry((a as u16) < b as u16 + carry as u16);
    result
}

/// CP: a subtraction that only sets flags. Bits 5 and 3 come from the operand.
pub fn cp8(f: &mut Flags, a: u8, b: u8) {
    sub8(f, a, b, false);
    f.set_bit5(b & 0x20 != 0);
    f.set_bit3(b & 0x08 != 0);
}

/// NEG.
pub fn neg8(f: &mut Flags, a: u8) -> u8 {
    sub8(f, 0, a, false)
}

pub fn and8(f: &mut Flags, a: u8, b: u8) -> u8 {
    let result = a & b;
    sz53p(f, result);
    f.set_half_carry(true);
    f.set_carry(false);
    result
}

pub fn xor8(f: &mut Flags, a: u8, b: u8) -> u8 {
    let result = a ^ b;
    sz53p(f, result);
    f.set_carry(false);
    result
}

pub fn or8(f: &mut Flags, a: u8, b: u8) -> u8 {
    let result = a | b;
    sz53p(f, result);
    f.set_carry(false);
    result
}

/// INC r: leaves C alone.
pub fn inc8(f: &mut Flags, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    f.set_sign(result & 0x80 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry(value & 0x0F == 0x0F);
    f.set_bit3(result & 0x08 != 0);
    f.set_parity_overflow(value == 0x7F);
    f.set_add_sub(false);
    result
}

/// DEC r: leaves C alone.
pub fn dec8(f: &mut Flags, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    f.set_sign(result & 0x80 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry(value & 0x0F == 0);
    f.set_bit3(result & 0x08 != 0);
    f.set_parity_overflow(value == 0x80);
    f.set_add_sub(true);
    result
}

/// ADD HL,rr (and IX, IY): leaves S, Z and P/V alone. H is the carry out of bit
/// 11, and bits 5 and 3 come from the result's high byte.
pub fn add16(f: &mut Flags, a: u16, b: u16) -> u16 {
    let sum = a as u32 + b as u32;
    let result = sum as u16;
    f.set_bit5(result & 0x2000 != 0);
    f.set_half_carry((a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF);
    f.set_bit3(result & 0x0800 != 0);
    f.set_add_sub(false);
    f.set_carry(sum > 0xFFFF);
    result
}

/// ADC HL,rr.
pub fn adc16(f: &mut Flags, a: u16, b: u16, carry: bool) -> u16 {
    let sum = a as u32 + b as u32 + carry as u32;
    let result = sum as u16;
    f.set_sign(result & 0x8000 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x2000 != 0);
    f.set_half_carry((a & 0x0FFF) + (b & 0x0FFF) + carry as u16 > 0x0FFF);
    f.set_bit3(result & 0x0800 != 0);
    f.set_parity_overflow((a ^ result) & (b ^ result) & 0x8000 != 0);
    f.set_add_sub(false);
    f.set_carry(sum > 0xFFFF);
    result
}

/// SBC HL,rr.
pub fn sbc16(f: &mut Flags, a: u16, b: u16, carry: bool) -> u16 {
    let result = a.wrapping_sub(b).wrapping_sub(carry as u16);
    f.set_sign(result & 0x8000 != 0);
    f.set_zero(result == 0);
    f.set_bit5(result & 0x2000 != 0);
    f.set_half_carry((a & 0x0FFF) < (b & 0x0FFF) + carry as u16);
    f.set_bit3(result & 0x0800 != 0);
    f.set_parity_overflow((a ^ b) & (a ^ result) & 0x8000 != 0);
    f.set_add_sub(true);
    f.set_carry((a as u32) < b as u32 + carry as u32);
    result
}

/// The CB-prefixed rotates and shifts: S, Z, 5, 3 and P/V from the result, H and
/// N cleared, C the bit shifted out.
fn shifted(f: &mut Flags, result: u8, carry: bool) -> u8 {
    sz53p(f, result);
    f.set_carry(carry);
    result
}

pub fn rlc8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value.rotate_left(1), value & 0x80 != 0)
}

pub fn rrc8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value.rotate_right(1), value & 0x01 != 0)
}

pub fn rl8(f: &mut Flags, value: u8) -> u8 {
    let carry = f.carry() as u8;
    shifted(f, value << 1 | carry, value & 0x80 != 0)
}

pub fn rr8(f: &mut Flags, value: u8) -> u8 {
    let carry = f.carry() as u8;
    shifted(f, value >> 1 | carry << 7, value & 0x01 != 0)
}

pub fn sla8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value << 1, value & 0x80 != 0)
}

pub fn sra8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value >> 1 | value & 0x80, value & 0x01 != 0)
}

/// SLL (undocumented): shifts left and sets bit 0.
pub fn sll8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value << 1 | 0x01, value & 0x80 != 0)
}

pub fn srl8(f: &mut Flags, value: u8) -> u8 {
    shifted(f, value >> 1, value & 0x01 != 0)
}

/// RLCA, RRCA, RLA and RRA rotate A like their CB-prefixed forms, but leave S, Z
/// and P/V alone.
fn rotated_a(f: &mut Flags, result: u8, carry: bool) -> u8 {
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry(false);
    f.set_bit3(result & 0x08 != 0);
    f.set_add_sub(false);
    f.set_carry(carry);
    result
}

pub fn rlca(f: &mut Flags, a: u8) -> u8 {
    rotated_a(f, a.rotate_left(1), a & 0x80 != 0)
}

pub fn rrca(f: &mut Flags, a: u8) -> u8 {
    rotated_a(f, a.rotate_right(1), a & 0x01 != 0)
}

pub fn rla(f: &mut Flags, a: u8) -> u8 {
    let carry = f.carry() as u8;
    rotated_a(f, a << 1 | carry, a & 0x80 != 0)
}

pub fn rra(f: &mut Flags, a: u8) -> u8 {
    let carry = f.carry() as u8;
    rotated_a(f, a >> 1 | carry << 7, a & 0x01 != 0)
}

/// BIT n: Z and P/V set if the bit is clear, S if it is bit 7 and set. Bits 5 and
/// 3 are copied from `xy`: the register for BIT n,r, and the high byte of the
/// address for BIT n,(IX+d). BIT n,(HL) copies them from the internal WZ
/// register, which isn't modelled, so it passes `None` and they're left alone.
pub fn bit(f: &mut Flags, n: u8, value: u8, xy: Option<u8>) {
    let set = value & (1 << n) != 0;
    f.set_sign(n == 7 && set);
    f.set_zero(!set);
    if let Some(xy) = xy {
        f.set_bit5(xy & 0x20 != 0);
        f.set_bit3(xy & 0x08 != 0);
    }
    f.set_half_carry(true);
    f.set_parity_overflow(!set);
    f.set_add_sub(false);
}

/// CPL: H and N set, bits 5 and 3 from the result.
pub fn cpl(f: &mut Flags, a: u8) -> u8 {
    let result = !a;
    f.set_bit5(result & 0x20 != 0);
    f.set_half_carry(true);
    f.set_bit3(result & 0x08 != 0);
    f.set_add_sub(true);
    result
}

/// SCF and CCF take bits 5 and 3 from `(Q ^ F) | A`, where Q is F if the previous
/// instruction set the flags and 0 if it didn't. Q isn't modelled, so they take
/// them from `F | A`, which is right when the previous instruction left F alone.
fn scf_ccf_bits_5_3(f: &mut Flags, a: u8) {
    let xy = u8::from(*f) | a;
    f.set_bit5(xy & 0x20 != 0);
    f.set_bit3(xy & 0x08 != 0);
}

/// SCF: sets C, clears H and N.
pub fn scf(f: &mut Flags, a: u8) {
    scf_ccf_bits_5_3(f, a);
    f.set_half_carry(false);
    f.set_add_sub(false);
    f.set_carry(true);
}

/// CCF: complements C, and H takes C's old value.
pub fn ccf(f: &mut Flags, a: u8) {
    scf_ccf_bits_5_3(f, a);
    let carry = f.carry();
    f.set_half_carry(carry);
    f.set_add_sub(false);
    f.set_carry(!carry);
}

/// LDI, LDD, LDIR and LDDR: P/V set while BC (after its decrement) isn't 0, H
/// and N cleared. Bits 5 and 3 are bits 1 and 3 of A plus the byte copied.
pub fn block_ld(f: &mut Flags, a: u8, value: u8, bc: u16) {
    let n = a.wrapping_add(value);
    f.set_bit5(n & 0x02 != 0);
    f.set_half_carry(false);
    f.set_bit3(n & 0x08 != 0);
    f.set_parity_overflow(bc != 0);
    f.set_add_sub(false);
}

/// When LDIR, LDDR, CPIR or CPDR repeats, bits 5 and 3 come from the high byte of
/// PC, the instruction's own address, instead.
pub fn block_repeat(f: &mut Flags, pc: u16) {
    let high = (pc >> 8) as u8;
    f.set_bit5(high & 0x20 != 0);
    f.set_bit3(high & 0x08 != 0);
}

/// CPI, CPD, CPIR and CPDR: S, Z and H as A - value sets them, N set, and P/V
/// set while BC (after its decrement) isn't 0. Bits 5 and 3 are bits 1 and 3 of
/// A - value - H. C is left alone.
pub fn block_cp(f: &mut Flags, a: u8, value: u8, bc: u16) {
    let result = a.wrapping_sub(value);
    let half_carry = a & 0x0F < value & 0x0F;
    let n = result.wrapping_sub(half_carry as u8);
    f.set_sign(result & 0x80 != 0);
    f.set_zero(result == 0);
    f.set_bit5(n & 0x02 != 0);
    f.set_half_carry(half_carry);
    f.set_bit3(n & 0x08 != 0);
    f.set_parity_overflow(bc != 0);
    f.set_add_sub(true);
}

/// INI, IND, OUTI, OUTD and their repeating forms: S, Z, 5 and 3 come from B
/// (after its decrement), and N from bit 7 of the byte moved. `k` is that byte
/// plus C + 1 (INI, INIR), C - 1 (IND, INDR) or L after HL steps (the OUT
/// forms): H and C are set if it passes 255, and P/V is the parity of its low
/// 3 bits XOR B.
pub fn block_io(f: &mut Flags, value: u8, k: u16, b: u8) {
    f.set_sign(b & 0x80 != 0);
    f.set_zero(b == 0);
    f.set_bit5(b & 0x20 != 0);
    f.set_half_carry(k > 0xFF);
    f.set_bit3(b & 0x08 != 0);
    f.set_parity_overflow(parity((k as u8 & 0x07) ^ b));
    f.set_add_sub(value & 0x80 != 0);
    f.set_carry(k > 0xFF);
}

/// While INIR, INDR, OTIR and OTDR repeat, bits 5 and 3 come from PC's high byte,
/// as for LDIR, and H and P/V change too, depending on C, the byte moved and B.
pub fn block_io_repeat(f: &mut Flags, pc: u16, value: u8, b: u8) {
    block_repeat(f, pc);
    // P/V flips when the low 3 bits of `x` have odd parity.
    let flip_pv = |f: &mut Flags, x: u8| {
        let pv = f.parity_overflow();
        f.set_parity_overflow(pv ^ !parity(x & 0x07));
    };
    if f.carry() {
        if value & 0x80 != 0 {
            flip_pv(f, b.wrapping_sub(1));
            f.set_half_carry(b & 0x0F == 0x00);
        } else {
            flip_pv(f, b.wrapping_add(1));
            f.set_half_carry(b & 0x0F == 0x0F);
        }
    } else {
        flip_pv(f, b);
    }
}

/// DAA: corrects A to packed BCD after an addition, or after a subtraction if N
/// is set.
pub fn daa(f: &mut Flags, a: u8) -> u8 {
    let low = f.half_carry() || a & 0x0F > 9;
    let high = f.carry() || a > 0x99;
    let mut correction = 0;
    if low {
        correction |= 0x06;
    }
    if high {
        correction |= 0x60;
    }
    let subtract = f.add_sub();
    let (result, half_carry) = if subtract {
        (a.wrapping_sub(correction), f.half_carry() && a & 0x0F < 6)
    } else {
        (a.wrapping_add(correction), a & 0x0F > 9)
    };
    sz53p(f, result);
    f.set_half_carry(half_carry);
    f.set_add_sub(subtract);
    f.set_carry(high);
    result
}
