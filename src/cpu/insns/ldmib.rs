use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 405
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldmib(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        for op in arm.operands() {
            assert!(op.ty == ARMOpType::ARM_OP_REG);
        }
        assert!(!arm.update_flags);
        self.assert_exception_return(insn);

        let mut address = self.op_value(&arm.operands()[0]).0 + 4;
        let mut n_is_in_list = false;
        let n = ::util::reg_num(arm.operands()[0].data());
        let len = arm.operands().len() - 1; // w/o base register
        for i in 1..(len + 1) {
            let r = ::util::reg_num(arm.operands()[i].data());
            if r == n {
                n_is_in_list = true;
            }

            assert!(r != 15);
            let value = self.mem.read(address as usize);
            self.set_reg(r, value);
            address += 4;
        }

        if arm.writeback && !n_is_in_list {
            let val = self.get_reg(n) + (4 * len) as u32;
            self.set_reg(n, val);
        }

        None
    }
}
