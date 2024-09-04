pub mod add_a_b;
pub mod add_a_c;
pub mod add_a_e;
pub mod add_a_phl;
pub mod add_hl_bc;
pub mod add_hl_de;
pub mod add_hl_hl;
pub mod add_ix_sp;

macro_rules! add_rr_rr {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let result = $reg1.wrapping_add($reg2);
        let carry = result < *$reg1;
        // check for carry between bits 11 and 12
        let half_carry = ((*$reg1 & 0x0fff) + ($reg2 & 0x0fff)) > 0x0fff;
        *$reg1 = result;
        $flags.set_carry(carry);
        $flags.set_half_carry(half_carry);
        $flags.set_add_sub(false);
        //set undocumented flags
        $flags.set_bit3((result >> 11) & 1 == 1);
        $flags.set_bit5((result >> 13) & 1 == 1);
    };
}

pub(crate) use add_rr_rr;

macro_rules! add_r_r {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let value_before = *$reg1;
        let result = value_before.wrapping_add($reg2);

        // calculate flags
        let carry = result < value_before;
        let half_carry = (value_before & 0x0f) + ($reg2 & 0x0f) > 0x0f;
        let zero = result == 0;
        let sign = (result & 0x80) != 0;
        let overflow =
            (value_before & 0x80) == ($reg2 & 0x80) && (value_before & 0x80) != (result & 0x80);
        *$reg1 = result;

        // set flags
        $flags.set_zero(zero);
        $flags.set_parity_overflow(overflow);
        $flags.set_sign(sign);
        $flags.set_half_carry(half_carry);
        $flags.set_add_sub(false);
        $flags.set_carry(carry);

        //set undocumented flags
        $flags.set_bit3((result >> 3) & 1 == 1);
        $flags.set_bit5((result >> 5) & 1 == 1);
    };
}

pub(crate) use add_r_r;
