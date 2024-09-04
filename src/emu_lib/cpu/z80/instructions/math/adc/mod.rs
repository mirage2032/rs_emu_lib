pub mod adc_a_b;
pub mod adc_a_d;
pub mod adc_a_n;

macro_rules! adc_rr_rr {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let result = $reg1
            .wrapping_add($reg2)
            .wrapping_add($flags.carry() as u16);
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

pub(crate) use adc_rr_rr;

macro_rules! adc_r_r {
    ($reg1:expr, $reg2:expr, $flags:expr) => {
        let value_before = *$reg1;
        let result = value_before
            .wrapping_add($reg2)
            .wrapping_add($flags.carry() as u8);

        // calculate flags
        let carry = (value_before as u16 + $reg2 as u16 + $flags.carry() as u16) > 0xFF;
        let half_carry = (value_before & 0x0f) + ($reg2 & 0x0f) + $flags.carry() as u8 > 0x0f;
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

pub(crate) use adc_r_r;
