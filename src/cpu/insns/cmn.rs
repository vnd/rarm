use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 364 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_cmn(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.writeback);
        assert!(arm.update_flags);
        ::util::assert_shift(arm.operands());
        let val = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_REG |
            ARMOpType::ARM_OP_IMM => self.op_value(&arm.operands()[1]).0,
            _ => { unreachable!() }
        };

        let rn = self.op_value(&arm.operands()[0]).0;
        let (result, carry, overflow) = ::arith::add_with_carry(rn, val, 0);

        self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_V, overflow );

        None
    }
}
