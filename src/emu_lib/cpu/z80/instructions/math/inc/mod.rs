pub mod inc_b;
pub mod inc_bc;
pub mod inc_c;

macro_rules! inc_r {
    ($reg:expr, $flags:expr) => {
        let sign = (*$reg & (1 << 7)) != 0;
        let half_carry = (*$reg & (1 << 4)) != 0;

        *$reg = $reg.wrapping_add(1);

        let newsign = (*$reg & (1 << 7)) != 0;
        let new_half_carry = (*$reg & (1 << 4)) != 0;

        $flags.set_zero(*$reg == 0);
        $flags.set_parity_overflow(sign != newsign);
        $flags.set_sign((*$reg & (1 << 7)) != 0);
        $flags.set_half_carry(half_carry != new_half_carry);
        $flags.set_add_sub(true);
    };
}
pub(crate) use inc_r;
