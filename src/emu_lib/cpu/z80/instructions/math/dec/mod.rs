pub mod dec_b;
pub mod dec_bc;
pub mod dec_c;
pub mod dec_d;
pub mod dec_de;
pub mod dec_e;
pub mod dec_h;
pub mod dec_hl;
pub mod dec_l;

macro_rules! dec_r {
    ($reg:expr, $flags:expr) => {
        let value_before = *$reg;
        let result = value_before.wrapping_sub(1);
        *$reg = result;

        let sign = (result & (1 << 7)) != 0;
        let half_carry = (value_before & 0x0F) == 0x00 && (result & 0x0F) == 0x0F;
        let pv_flag = value_before == 0x80;

        // Update flags
        $flags.set_zero(result == 0);
        $flags.set_parity_overflow(pv_flag); // Correctly combine parity and overflow
        $flags.set_sign(sign);
        $flags.set_half_carry(half_carry);
        $flags.set_add_sub(true); // DEC is a subtraction, so set to true

        // Set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}

pub(crate) use dec_r;
