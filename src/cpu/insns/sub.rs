use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 710 (imm)
   p. 712 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_sub(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 3);
        ::util::assert_shift(&arm.operands()[0..2]);

        let val = match arm.operands()[2].ty {
            ARMOpType::ARM_OP_IMM => ::util::imm_to_u32(arm.operands()[2].data()),
            ARMOpType::ARM_OP_REG => self.op_value(&arm.operands()[2]).0,
            _ => { assert!(false); 0 },
        };
        assert!(false == arm.writeback);

        let rn = self.op_value(&arm.operands()[1]).0;
        let (result, carry, overflow) = ::arith::add_with_carry(rn, !val, 1);

        let d = ::util::reg_num(arm.operands()[0].data());
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
