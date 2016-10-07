use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;
use std::mem::transmute;

/* p. 516 (imm)
   p. 518 (reg)
   p. 520 (rsr)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_orr(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..2]);
        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let raw: u32 = self.mem.read(insn.address as usize);
        let (val, carry) = match arm.operands()[2].ty {
            ARMOpType::ARM_OP_REG => {
                if arm.operands()[2].shifter() as u32 >= ARMShifter::ARM_SFT_ASR_REG as u32 {
                    let shift_reg = ::util::cs_reg_num(transmute(arm.operands()[2].shift_value));
                    let shift_reg_val = self.get_reg(shift_reg);
                    self.set_reg(shift_reg, shift_reg_val & 0xFF);
                    let ret = self.op_value(&arm.operands()[2]);
                    self.set_reg(shift_reg, shift_reg_val);
                    ret
                } else {
                    self.op_value(&arm.operands()[2])
                }
            },
            ARMOpType::ARM_OP_IMM => expand_imm_c(::util::get_bits(raw, 0..11), self.get_carry()),
            _ => { assert!(false); (0, 0) },
        };

        let result = self.get_reg(n) | val;
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
