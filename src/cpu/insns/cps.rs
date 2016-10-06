use capstone::ffi::*;
use capstone::ffi::detail::*;
use ::util::{get_bit, get_bits};
use ::cpu::cpsr::*;

// p. 1981
// implemented by parsing raw instruction since:
// 1. 'effect' seems to be confused with 'mode' by capstone
// 2. apparently no multiple flags support
impl ::cpu::core::CPU {
    pub unsafe fn exec_cps(&mut self, insn: &Insn) -> Option<u32> {
        let raw: u32 = self.mem.read(insn.address as usize);
        let mode = get_bits(raw, 0..4);
        let m = get_bit(raw, 17);
        let a = get_bit(raw, 8);
        let i = get_bit(raw, 7);
        let f = get_bit(raw, 6);
        let imod = get_bits(raw, 18..19);
        let enable = imod == 0b10;
        let disable = imod == 0b11;
        let changemode = m == 1;
        assert!(!(mode != 0 && m == 0));
        assert!(!(get_bit(raw, 19) == 1 && get_bits(raw, 6..8) == 0));
        assert!(!(get_bit(raw, 19) == 0 && get_bits(raw, 6..8) != 0));
        assert!(!(imod == 0b00 && m == 0));
        assert!(imod != 0b01);
        let arm = insn.detail().unwrap().data_arm();
        assert!(arm.cc == ARMCC::ARM_CC_AL);
        assert!(!changemode && mode == 0); // CPU mode
        if enable {
                if a == 1 {
                    self.set_cpsr_bit(CPSR_A, 0);
                };
                if i == 1 {
                    self.set_cpsr_bit(CPSR_I, 0);
                };
                if f == 1 {
                    self.set_cpsr_bit(CPSR_F, 0);
                };
        };
        if disable {
                if a == 1 {
                    self.set_cpsr_bit(CPSR_A, 1);
                };
                if i == 1 {
                    self.set_cpsr_bit(CPSR_I, 1);
                };
                if f == 1 {
                    self.set_cpsr_bit(CPSR_F, 1);
                };
        };
        if changemode {
            self.set_cpsr_bits(CPSR_M, mode);
        };
        None
    }
}
