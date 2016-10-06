use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 480 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_mla(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 4);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[3].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let m = ::util::reg_num(arm.operands()[2].data());
        let a = ::util::reg_num(arm.operands()[3].data());
        assert!(m != 15 && n != 15 && m != 15 && a != 15);

        let operand1 = self.get_reg(n) as i32;
        let operand2 = self.get_reg(m) as i32;
        let addend = self.get_reg(a) as i32;
        let result = (operand1 * operand2 + addend) as u32;

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        }
        None
    }
}
