macro_rules! cp_r_setf {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let value_before = $reg1;
        let result = value_before.wrapping_sub($reg2);

        // Calculate flags
        let sign = (result & 0x80) != 0;
        let zero = result == 0;

        // Half Carry: Detects borrow from bit 4
        let half_carry = ((value_before & 0x0F) < ($reg2 & 0x0F));

        // Overflow: Detects overflow if the result has an incorrect sign
        let overflow = ((value_before ^ $reg2) & (value_before ^ result) & 0x80) != 0;

        // Carry: Detects borrow from bit 8
        let carry = (value_before as u16) < ($reg2 as u16);

        // Set flags
        $flags.set_sign(sign);
        $flags.set_zero(zero);
        $flags.set_half_carry(half_carry);
        $flags.set_parity_overflow(overflow);
        $flags.set_add_sub(true); // Always set for subtraction
        $flags.set_carry(carry);

        // Set undocumented flags
        $flags.set_bit3(($reg2 >> 3) & 1 == 1);
        $flags.set_bit5(($reg2 >> 5) & 1 == 1);
    };
}
pub(crate) use cp_r_setf;

pub mod cp_r;
