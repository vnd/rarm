use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 734 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_sxth(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let rm = self.op_value(&arm.operands()[1]).0;
        self.set_reg(d, ::arith::sign_extend_u16(::util::get_bits(rm, 0..15) as u16));

        None
    }
}
