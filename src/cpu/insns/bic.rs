use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;

/* p. 340 (imm)
   p. 344 (rsr)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_bic(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..2]);
        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        assert!(n != 15);

        let raw: u32 = self.mem.read(insn.address as usize);
        let (val, carry) = match arm.operands()[2].ty {
            ARMOpType::ARM_OP_IMM => expand_imm_c(::util::get_bits(raw, 0..11), self.get_carry()),
            ARMOpType::ARM_OP_REG => (self.op_value(&arm.operands()[2]), self.get_carry()).0,
            _ => unreachable!(),
        };
        let result = self.get_reg(n) & !val;

        if d == 15 {
            assert!(false); // rsr form does not support PC
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
