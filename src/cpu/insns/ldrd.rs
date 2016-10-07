use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 428 (literal)
   p. 430 (register)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrd(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..1]);
        ::util::check_subtracted(arm.operands(), insn);
        assert!(!arm.writeback);
        assert!(!arm.update_flags);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_MEM);
        assert!(arm.operands().len() == 3);
        let raw: u32 = self.mem.read(insn.address as usize);
        let index = ::util::get_bit(raw, 24);
        let add = ::util::get_bit(raw, 23) == 1;
        assert!(index == 1);

        let t = ::util::reg_num(arm.operands()[0].data());
        let t2 = ::util::reg_num(arm.operands()[1].data());
        assert!(t != 15 && t2 != 15 && t2 == t + 1);
        let addr = self._op_value(&arm.operands()[2], add).0 as usize;
        let data  = self.mem.read(addr);
        let data2 = self.mem.read(addr + 4);
        self.set_reg(t,  data);
        self.set_reg(t2, data2);

        None
    }
}
