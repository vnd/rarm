use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 348 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_bl(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 1);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_IMM);
        assert!(false == arm.update_flags);
        assert!(false == arm.writeback);

        let pc = self.get_reg(15);
        self.set_reg(14, pc - 4);
        Some(arm.operands()[0].data[0] as u32)
    }
}
