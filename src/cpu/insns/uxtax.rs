use capstone::ffi::*;
use capstone::ffi::detail::*;

impl ::cpu::core::CPU {
    pub unsafe fn exec_uxtax(&mut self, insn: &Insn, bytes: u32) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands().len() == 3);
        assert!(!arm.writeback);
        assert!(!arm.update_flags);
        ::util::assert_shift(arm.operands());
        let raw: u32 = self.mem.read(insn.address as usize);
        assert!(::util::get_bits(raw, 10..11) == 0);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let m = ::util::reg_num(arm.operands()[2].data());

        let rn = self.get_reg(n);
        let rm = self.get_reg(m);
        self.set_reg(d, rn + ::util::get_bits(rm, 0..(8*bytes-1)));

        None
    }
}
