use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 502 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_mul(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let m = ::util::reg_num(arm.operands()[2].data());
        assert!(n != 15 && m != 15);

        let rn = self.get_reg(n);
        let rm = self.get_reg(m);
        let result = rn * rm;

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        }
        None
    }
}
