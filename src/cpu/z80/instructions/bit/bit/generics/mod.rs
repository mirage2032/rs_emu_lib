pub mod bit_b_phl;
pub mod bit_b_pixd;
pub mod bit_b_piyd;
pub mod bit_b_r;

macro_rules! bit_b_r_setf {
    ($reg1:expr, $bit:expr, $flags:expr) => {
        let test_bit = ($reg1 >> $bit) & 1 == 1;
        $flags.set_zero(!test_bit);
        $flags.set_half_carry(true);
        $flags.set_add_sub(false);
        if $bit == 7 {
            $flags.set_sign(test_bit);
        } else {
            $flags.set_sign(false);
        }
        $flags.set_parity_overflow(!test_bit);
    };
}
pub(crate) use bit_b_r_setf;
