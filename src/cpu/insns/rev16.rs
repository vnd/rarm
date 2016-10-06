use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 564 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_rev16(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);

        let d = ::util::reg_num(arm.operands()[0].data());
        let m = ::util::reg_num(arm.operands()[1].data());
        assert!(d != 15 && m != 15);

        let rm = self.get_reg(m);
        let mut result = 0;
        ::util::set_bits(&mut result, 24..31, ::util::get_bits(rm, 16..23));
        ::util::set_bits(&mut result, 16..23, ::util::get_bits(rm, 24..31));
        ::util::set_bits(&mut result,  8..15, ::util::get_bits(rm, 0..7));
        ::util::set_bits(&mut result,   0..7, ::util::get_bits(rm, 8..15));
        self.set_reg(d, result);

        None
    }
}
