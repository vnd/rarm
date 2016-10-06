use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 432
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrex(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands());
        ::util::check_subtracted(&arm.operands(), insn);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_MEM);
        assert!(arm.operands().len() == 2);

        let t = ::util::reg_num(arm.operands()[0].data());
        assert!(t != 15);
        let data = self.mem.read(self.op_value(&arm.operands()[1]).0 as usize);
        self.set_reg(t, data);

        None
    }
}
