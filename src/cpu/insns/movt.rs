use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 491
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_movt(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_IMM);
        assert!(false == arm.update_flags);
        assert!(false == arm.writeback);
        ::util::assert_shift(&arm.operands());

        let d = ::util::reg_num(arm.operands()[0].data());
        assert!(d != 15);

        let mut val = self.get_reg(d);
        let imm = ::util::imm_to_u32(arm.operands()[1].data());
        assert!(imm < 2u32.pow(16));
        ::util::set_bits(&mut val, (16..31), imm);
        self.set_reg(d, val);

        None
    }
}
