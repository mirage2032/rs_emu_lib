pub mod inc_b;
pub mod inc_bc;
pub mod inc_c;
pub mod inc_d;
pub mod inc_de;
pub mod inc_e;

macro_rules! inc_r {
    ($reg:expr, $flags:expr) => {
        let value_before = *$reg;
        let result = value_before.wrapping_add(1);
        *$reg = result;

        let sign = (result & (1 << 7)) != 0;
        let half_carry = (value_before & 0x0F) == 0x0F;
        let pv_flag = value_before == 0x7F;

        // Update flags
        $flags.set_zero(result == 0);
        $flags.set_parity_overflow(pv_flag); // Correctly combine parity and overflow
        $flags.set_sign(sign);
        $flags.set_half_carry(half_carry);
        $flags.set_add_sub(false); // INC is an addition, so set to false

        // Set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}
pub(crate) use inc_r;
