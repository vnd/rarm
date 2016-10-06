use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 664
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_stm(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        for op in arm.operands() {
            assert!(op.ty == ARMOpType::ARM_OP_REG);
        }
        assert!(false == arm.update_flags);
        self.assert_exception_return(insn);

        let len = arm.operands().len() - 1; // w/o base register
        let mut address = self.op_value(&arm.operands()[0]).0;
        let n = ::util::reg_num(arm.operands()[0].data());
        for i in 1..(len + 1) {
            let r = ::util::reg_num(arm.operands()[i].data());
            assert!(r != n);
            assert!(r != 15);
            if r != 15 {
                let val = self.get_reg(r) as usize;
                self.mem.write(address as usize, val);
            } else {
                assert!(false);
            }
            address = address + 4;
        }

        if arm.writeback {
            let val = self.get_reg(n);
            self.set_reg(n, val + (4 * len) as u32);
        }

        None
    }
}
