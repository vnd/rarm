use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 335 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_b(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 1);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_IMM);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        Some(arm.operands()[0].data[0] as u32)
    }
}
