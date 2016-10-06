use capstone::ffi::detail::{ARMShifter};

// p. 41
pub fn lsl_c(value: u32, shift: u32) -> (u32, u32) {
    assert!(shift > 0);

    let result = (value as u64) << shift;
    let carry_out = match result & (1 << 32) {
        0 => 0,
        _ => 1,
    };
    (result as u32, carry_out)
}

// p. 42
pub fn lsl(value: u32, shift: u32) -> u32 {
    if shift == 0 {
        return value;
    }

    let (result, _) = lsl_c(value, shift);
    result
}

// p. 42
pub fn lsr_c(value: u32, shift: u32) -> (u32, u32) {
    assert!(shift > 0);

    let ret = value >> shift;
    if value > 0 && ret == value {
        println!("lsr_c: 0x{:x} >> 0x{:x} == 0x{:x}", value, shift, ret);
        assert!(shift == 32);
        return (0, ::util::get_bit(value, 31));
    }
    let carry = match shift {
        1...32 => ::util::get_bit(value, shift - 1),
        _ => 0,
    };
    (ret, carry)
}

// p. 42
pub fn ror_c(x: u32, shift: u32) -> (u32, u32) {
    let n = 32;

    assert!(shift > 0 && shift <= n);

    let m = shift;// MOD n;
    let result = ::arith::lsr(x, m) | ::arith::lsl(x, n - m);
    let carry_out = ::util::get_bit(result, n - 1);
    (result, carry_out)
}

// p. 42
pub fn asr_c(x: u32, shift: u32) -> (u32, u32) {
    let n = 32;

    assert!(shift > 0 && shift <= n);
    let carry_out = ::util::get_bit(x, shift - 1);
    ((x as i32 >> shift) as u32, carry_out)
}

// p. 43
pub fn rrx_c(x: u32, carry_in: u32) -> (u32, u32) {
    let n = 32;
    assert!(carry_in == 0 || carry_in == 1);

    let mut result = x >> 1;
    ::util::set_bit(&mut result, n - 1, carry_in);

    let carry_out = ::util::get_bit(result, 0);
    (result, carry_out)
}

// p. 42
pub fn lsr(value: u32, shift: u32) -> u32 {
    if shift == 0 {
        return value;
    }

    let (result, _) = lsr_c(value, shift);
    result
}

// p. 292
// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0497a/CIHDDCIF.html
pub fn shift_c(value: u32, shifter: ARMShifter, shift_value: u32, carry_in: u32) -> (u32, u32) {
    assert!(!(shifter == ARMShifter::ARM_SFT_RRX_REG && shift_value != 1));
    assert!(carry_in == 0 || carry_in == 1);

    if shift_value == 0 || shifter == ARMShifter::ARM_SFT_INVALID {
        return (value, carry_in)
    }

    match shifter {
        ARMShifter::ARM_SFT_LSL     => lsl_c(value, shift_value),
        ARMShifter::ARM_SFT_LSR     => lsr_c(value, shift_value),
        ARMShifter::ARM_SFT_ROR     => ror_c(value, shift_value),
        ARMShifter::ARM_SFT_ASR     => asr_c(value, shift_value),
        ARMShifter::ARM_SFT_RRX     => rrx_c(value, carry_in),
        _ => { println!("{:#?}", shifter); unreachable!() },
    }
}

// p. 43
pub fn add_with_carry(x: u32, y: u32, carry: u32) -> (u32, u32, u32) {
    assert!(carry == 0 || carry == 1);

    let uyc = y.overflowing_add(carry);
    let unsigned_sum = x.overflowing_add(uyc.0);
    let carry_out = if unsigned_sum.1 || uyc.1 { 1 } else { 0 };

    let syc = (y as i32).overflowing_add(carry as i32);
    let signed_sum = (x as i32).overflowing_add(syc.0);
    let overflow  = if   signed_sum.1 || syc.1 { 1 } else { 0 };

    (unsigned_sum.0, carry_out, overflow)
}

// p. 201
pub fn expand_imm_c(imm: u32, carry_in: u32) -> (u32, u32) {
    let unrotated_value = ::util::get_bits(imm, (0..7));
    ::arith::shift_c(unrotated_value, ARMShifter::ARM_SFT_ROR, 2*::util::get_bits(imm, (8..11)), carry_in)
}

pub fn count_leading_zero_bits(val: u32) -> u32 {
    val.leading_zeros()
}

pub fn sign_extend_u16(val: u16) -> u32 {
    val as i16 as i32 as u32
}

pub fn sign_extend_u32(val: u32) -> u32 {
    val as i32 as u32
}
