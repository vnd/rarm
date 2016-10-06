use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 690
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_strex(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands());
        ::util::check_subtracted(&arm.operands(), insn);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_MEM);
        assert!(arm.operands().len() == 3);

        let d = ::util::reg_num(arm.operands()[0].data());
        let t = ::util::reg_num(arm.operands()[1].data());
        assert!(t != 15);
        let address = self.op_value(&arm.operands()[2]).0 as usize;
        let rt = self.get_reg(t) as usize;
        self.mem.write(address, rt);
        self.set_reg(d, 0);

        None
    }
}
