use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 496 (register)

*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_mrs(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        assert!(d != 15);

        let spec_reg = match arm.operands()[1].data() {
            ARMOpData::Reg(r) => r,
            _ => unreachable!(),
        };

        let val = match spec_reg {
            ARMReg::ARM_REG_APSR => self.cpsr,
            ARMReg::ARM_REG_SPSR => self.get_spsr(),
            _ => { ::util::dump_insn(insn, true); println!("unreachable in msr: {:#?}", spec_reg); unreachable!() },
        };
        self.set_reg(d, val);
        None
    }
}
