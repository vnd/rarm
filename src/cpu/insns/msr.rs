use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 500, 1998 (register)
   p. 498, 1996 (imm)
   p. 1153 CPSRWriteByInstr()
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_msr(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_SYSREG);
        assert!(false == arm.update_flags);
        assert!(false == arm.writeback);

        let val = match arm.operands()[1].data() {
            ARMOpData::Reg(r) => { let n = ::util::cs_reg_num(r); assert!(n != 15); self.get_reg(n) },
            ARMOpData::Imm(i) => { i },
            _ => unreachable!(),
        };

        if arm.operands()[0].data() == ARMOpData::Sysreg(ARMSysreg::ARM_SYSREG_CPSR_C) {
            self.set_cpsr_bits(0..7, val); // only bits 0..7
        } else if arm.operands()[0].data[0] == 15 && arm.operands()[0].data[1] == 0 {
            self.set_spsr(val); // all SPSR bits
        } else {
            unreachable!();
        }


        None
    }
}
