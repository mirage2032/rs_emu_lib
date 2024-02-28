pub mod dec_b;
pub mod dec_bc;
pub mod dec_c;

#[macro_export]
macro_rules! dec_r {
    ($reg:expr, $flags:expr) => {
        let sign = (*$reg & (1<<7)) != 0;
        let half_carry = (*$reg & (1<<3)) != 0;
        
        *$reg = $reg.wrapping_sub(1);
        
        let newsign = (*$reg & (1<<7)) != 0;
        let new_half_carry = (*$reg & (1<<3)) != 0;
        
        $flags.set_zero(*$reg == 0);
        $flags.set_parity_overflow(sign != newsign);
        $flags.set_sign((*$reg & (1<<7)) != 0);
        $flags.set_half_carry(half_carry != new_half_carry);
        $flags.set_add_sub(true);
    };
}