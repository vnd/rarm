
use capstone::ffi::*;
use capstone::ffi::detail::*;

impl ::cpu::core::CPU {
    pub unsafe fn exec_rrx(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);
        ::util::assert_shift(&arm.operands());

        let d = ::util::reg_num(arm.operands()[0].data());
        let m = ::util::reg_num(arm.operands()[1].data());

        assert!(d != 15 && m != 15);

        let (result, carry) = ::arith::shift_c(self.get_reg(m), ARMShifter::ARM_SFT_RRX, 1, self.get_carry());

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            // APSR.V unchanged
        }
        None
    }
}
