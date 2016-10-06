use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 576 (reg)
   p. 574 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_rsb(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let rn = self.op_value(&arm.operands()[1]).0;
        let (shifted, _) = self.op_value(&arm.operands()[2]);
        let (result, carry, overflow) = ::arith::add_with_carry(!rn, shifted, 1);

        if d == 15 {
            assert!(false == arm.update_flags);
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
