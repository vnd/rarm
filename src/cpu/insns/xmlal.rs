use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 776 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_xmlal(&mut self, insn: &Insn, signed: bool) -> Option<u32> {
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
        assert!(!arm.writeback);

        let dlo = ::util::reg_num(arm.operands()[0].data());
        let dhi = ::util::reg_num(arm.operands()[1].data());
        let n   = ::util::reg_num(arm.operands()[2].data());
        let m   = ::util::reg_num(arm.operands()[3].data());
        assert!(dlo != 15 && dhi != 15 && n != 15 && m != 15);
        assert!(dlo != dhi);

        let rn = self.get_reg(n);
        let rm = self.get_reg(m);
        let rdlo = self.get_reg(dlo);
        let rdhi = self.get_reg(dhi);
        let result = if signed {
            ((rn as i64) * (rm as i64) + ((((rdhi as u64) << 32) + (rdlo as u64)) as i64)) as u64
        } else {
            (rn as u64) * (rm as u64) + ((rdhi as u64) << 32) + (rdlo as u64)
        };

        self.set_reg(dlo, (result & 0xFFFFFFFF) as u32);
        self.set_reg(dhi, (result >> 32) as u32);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit((result >> 32) as u32, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
        }
        None
    }
}
