macro_rules! and_r_setf {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let result = $reg1 & $reg2;
        $reg1 = result;

        // Set flags
        $flags.set_sign(result & 0x80 == 0x80);
        $flags.set_zero(result == 0);
        $flags.set_half_carry(true); // Always set to true
        $flags.set_parity_overflow(result.count_ones() as u8 % 2 == 0);
        $flags.set_add_sub(false); // Always set for subtraction
        $flags.set_carry(false); // Always set to false

        // Set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}
pub(crate) use and_r_setf;

pub mod and_r;
