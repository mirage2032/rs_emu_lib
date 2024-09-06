macro_rules! xor_r_r_setf {
    ($dst:expr, $src:expr, $flags:expr) => {
        let result = *$dst ^ *$src;
        let sign = (result >> 7) & 1 == 1;
        let parity = result.count_ones() % 2 == 0;
        *$dst = result;

        // Update flags
        $flags.set_zero(result == 0);
        $flags.set_parity_overflow(parity); // Correctly combine parity and overflow
        $flags.set_sign(sign);
        $flags.set_half_carry(false);
        $flags.set_add_sub(false);
        $flags.set_carry(false);

        // Set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}

pub(crate) use xor_r_r_setf;

pub mod xor_r;
