use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 372 (reg)
   p. 370 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_cmp(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.writeback);
        assert!(arm.update_flags);
        let val = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_REG =>  self.op_value(&arm.operands()[1]).0,
            ARMOpType::ARM_OP_IMM => {
                ::util::assert_shift(arm.operands());
                ::util::imm_to_u32(arm.operands()[1].data())
            },
            _ => { assert!(false); 0 }
        };

        let n = self.op_value(&arm.operands()[0]).0;

        let (result, carry, overflow) = ::arith::add_with_carry(n, !val, 1);

        self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_V, overflow );

        None
    }
}
