pub mod add_hl_bc;


macro_rules! add_rr_rr {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let result = $reg1.wrapping_add($reg2);
        let carry = result < *$reg1;
        let half_carry = ((*$reg1 & 0xFFF) + ($reg2 & 0xFFF)) & 0x1000 != 0;
        *$reg1 = result;
        $flags.set_carry(carry);
        $flags.set_half_carry(half_carry);
        $flags.set_add_sub(false);
    };
}

pub(crate) use add_rr_rr;
