pub mod add_hl_bc;

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
