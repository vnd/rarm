use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 339

*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_bfi(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 4);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
        assert!(arm.operands()[3].ty == ARMOpType::ARM_OP_IMM);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let lsb = match arm.operands()[2].data() {
            ARMOpData::Imm(imm) => imm,
            _ => unreachable!(),
        };
        let width = match arm.operands()[3].data() {
            ARMOpData::Imm(imm) => imm,
            _ => unreachable!(),
        };

        let val = ::util::get_bits(self.get_reg(n), 0..width-1);
        let mut new_rd = self.get_reg(d);
        ::util::set_bits(&mut new_rd, lsb..lsb+width-1, val);

        self.set_reg(d, new_rd);

        None
    }
}
