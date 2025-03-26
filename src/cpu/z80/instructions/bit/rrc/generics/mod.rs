macro_rules! rrc_r_setf {
    ($reg1:expr, $flags:expr) => {
        $flags.set_carry($reg1 & 1 == 1);

        let bit7 = $reg1.rotate_right(1) & 0x80;
        $reg1 = ($reg1 >> 1) | bit7;
        // Set flags
        $flags.set_sign($reg1 & 0x80 == 0x80);
        $flags.set_zero($reg1 == 0);
        $flags.set_half_carry(false); // Always set to true
        $flags.set_parity_overflow($reg1.count_ones() as u8 % 2 == 0);
        $flags.set_add_sub(false); // Always set for subtraction

        // Set undocumented flags
        $flags.set_bit3(($reg1 >> 3) & 1 == 1);
        $flags.set_bit5(($reg1 >> 5) & 1 == 1);
    };
}
pub(crate) use rrc_r_setf;

pub mod rrc_r;
