use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 382 (imm)
   p. 384 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_eor(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..1]);
        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);
        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());

        let (shifted, carry) = self.op_value(&arm.operands()[2]);
        let result = shifted ^ self.get_reg(n);

        if d == 15 {
            assert!(false == arm.update_flags);
            return Some(result);
        }

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            // CPSR_V unchanged
        }
        None
    }
}
