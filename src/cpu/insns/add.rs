use capstone::ffi::*;

/* p. 308 (imm)
   p. 312 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_add(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..2]);
        assert!(arm.operands().len() == 3);
        let val = self.op_value(&arm.operands()[2]);
        assert!(!arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let (result, carry, overflow) = ::arith::add_with_carry(self.get_reg(n), val.0, 0);

        if d == 15 {
            assert!(!arm.update_flags);
            return Some(result);
        }

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            self.set_cpsr_bit(::cpu::cpsr::CPSR_V, overflow);
        }
        None
    }
}
