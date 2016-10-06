use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;

/* p. 744 (imm)
   p. 746 (reg)
*/

impl ::cpu::core::CPU {
    pub unsafe fn exec_tst(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        let raw: u32 = self.mem.read(insn.address as usize);
        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        let (shifted, carry) = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_IMM => expand_imm_c(::util::get_bits(raw, 0..11), self.get_carry()),
            ARMOpType::ARM_OP_REG => self.op_value(&arm.operands()[1]),
            _ => { assert!(false); (0, 0) },
        };
        assert!(true == arm.update_flags);
        assert!(false == arm.writeback);

        let n = ::util::reg_num(arm.operands()[0].data());
        let result = self.get_reg(n) & shifted;

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            // APSR.V unchanged
        }
        None
    }
}
