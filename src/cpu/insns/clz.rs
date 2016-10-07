use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 362
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_clz(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands().len() == 2);
        assert!(!arm.writeback);
        assert!(!arm.update_flags);

        let d = ::util::reg_num(arm.operands()[0].data());
        let m = ::util::reg_num(arm.operands()[1].data());

        let result = ::arith::count_leading_zero_bits(self.get_reg(m));

        self.set_reg(d, result);

        None
    }
}
