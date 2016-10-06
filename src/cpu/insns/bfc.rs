use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 336 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_bfc(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_IMM);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);

        let d = ::util::reg_num(arm.operands()[0].data());
        assert!(d != 15);
        let lsb = ::util::imm_to_u32(arm.operands()[1].data());
        let width = ::util::imm_to_u32(arm.operands()[2].data());

        let mut rd = self.get_reg(d);
        ::util::set_bits(&mut rd, lsb..lsb+width-1, 0);
        self.set_reg(d, rd);

        None
    }
}
