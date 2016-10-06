use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 352 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_bx(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 1);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.update_flags);
        assert!(false == arm.writeback);

        let addr = self.op_value(&arm.operands()[0]).0;
        assert!(0 == ::util::get_bits(addr, (0..1)));
        Some(addr)
    }
}
