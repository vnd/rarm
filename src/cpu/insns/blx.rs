use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 351 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_blx(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 1);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        let pc = self.get_reg(15);
        self.set_reg(14, pc - 4);
        let reg = self.op_value(&arm.operands()[0]).0;
        assert!(::util::get_bits(reg, 0..1) == 0);
        Some(reg)
    }
}
