use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 332 (reg)
*/

impl ::cpu::core::CPU {
    pub unsafe fn exec_asr(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);
        ::util::assert_shift(&arm.operands());

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let shift_n = match arm.operands()[2].data() {
            ARMOpData::Reg(r) => ::util::get_bits(self.get_reg(::util::cs_reg_num(r)) as u32, 0..7),
            ARMOpData::Imm(i) => i,
            _ => { assert!(false); 0 },
        };

        assert!(d != 15 && n != 15);

        let (result, carry) = ::arith::shift_c(self.get_reg(n), ARMShifter::ARM_SFT_ASR, shift_n, self.get_carry());

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
