use capstone::ffi::*;
use capstone::ffi::detail::*;

impl ::cpu::core::CPU {
    pub unsafe fn exec_uxtx(&mut self, insn: &Insn, bytes: u32) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands().len() == 2);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);
        ::util::assert_shift(&arm.operands());

        let d = ::util::reg_num(arm.operands()[0].data());
        let m = ::util::reg_num(arm.operands()[1].data());

        let val = self.get_reg(m);
        let msbit = bytes * 8 - 1;
        self.set_reg(d, ::util::get_bits(val, (0..msbit)));

        None
    }
}
