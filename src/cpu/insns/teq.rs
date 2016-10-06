use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;

/* p. 738 (imm)
   p. 740 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_teq(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.writeback);

        let raw: u32 = self.mem.read(insn.address as usize);
        let rn = self.op_value(&arm.operands()[0]).0;
        let (shifted, carry) = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_REG => self.op_value(&arm.operands()[1]),
            ARMOpType::ARM_OP_IMM => expand_imm_c(::util::get_bits(raw, (0..11)), self.get_carry()),
            _ => unreachable!(),
        };

        let result = rn ^ shifted;

        assert!(arm.update_flags);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
        // CPSR.V unchanged

        None
    }
}
