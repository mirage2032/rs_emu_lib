macro_rules! sbc_r_r {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let reg2 = $reg2; //copy needed for sbc a,a to work
        let value_before = $reg1;
        let result = value_before
            .wrapping_sub(reg2)
            .wrapping_sub($flags.carry() as u8);
        $reg1 = result;

        // Calculate flags
        let sign = (result & 0x80) != 0;
        let zero = result == 0;
        let half_carry = ((value_before & 0x0F) < (reg2 & 0x0F) + $flags.carry() as u8);
        let overflow = ((value_before ^ reg2) & (value_before ^ result) & 0x80) != 0;
        let carry = (value_before as u16) < (reg2 as u16) + $flags.carry() as u16;

        // Set flags
        $flags.set_sign(sign);
        $flags.set_zero(zero);
        $flags.set_half_carry(half_carry);
        $flags.set_parity_overflow(overflow);
        $flags.set_add_sub(true); // Always set for subtraction
        $flags.set_carry(carry);

        // Set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}

macro_rules! sbc_rr_rr {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let reg2 = $reg2;
        let value_before = $reg1;
        let result = $reg1.wrapping_sub(reg2).wrapping_sub($flags.carry() as u16);
        $reg1 = result;

        // Calculate flags
        let half_carry = ((value_before & 0x0FFF) < (reg2 & 0x0FFF) + $flags.carry() as u16);
        let overflow = ((value_before ^ reg2) & (value_before ^ result) & 0x8000) != 0;
        let carry = (value_before as u32) < (reg2 as u32) + $flags.carry() as u32;

        // Set flags
        $flags.set_half_carry(half_carry);
        $flags.set_parity_overflow(overflow);
        $flags.set_add_sub(true); // Always set for subtraction
        $flags.set_carry(carry);
        $flags.set_zero(result == 0);
        $flags.set_sign((result & 0x8000) != 0);
    };
}

pub(crate) use sbc_r_r;
pub(crate) use sbc_rr_rr;

pub mod sbc_a_r;
