use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;

/* p. 504 (imm)
   p. 506 (reg)
   p. 508 (rsr)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_mvn(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        let (result, carry) = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_REG => {
                let val = self.op_value(&arm.operands()[1]).0;
                (!val, self.get_carry())
            },
            ARMOpType::ARM_OP_IMM => {
                ::util::assert_shift(arm.operands());
                let raw: u32 = self.mem.read(insn.address as usize);
                let (imm, carry) = expand_imm_c(::util::get_bits(raw, 0..11), self.get_carry());
                (!imm, carry)
            },
            _ => unreachable!()
        };

        let d = ::util::reg_num(arm.operands()[0].data());
        if d == 15 {
            assert!(!arm.update_flags);
            return Some(result);
        }

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            // APSR.V unchanged
        }
        None
    }
}
