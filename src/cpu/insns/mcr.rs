use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 476 */

impl ::cpu::core::CPU {
    pub unsafe fn exec_mcr(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 6);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_PIMM);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_IMM);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[3].ty == ARMOpType::ARM_OP_CIMM);
        assert!(arm.operands()[4].ty == ARMOpType::ARM_OP_CIMM);
        assert!(arm.operands()[5].ty == ARMOpType::ARM_OP_IMM);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        let coproc = ::util::imm_to_u32(arm.operands()[0].data());
        assert!(coproc == 15);
        let opc1   = ::util::imm_to_u32(arm.operands()[1].data());
        let rt     = self.op_value(&arm.operands()[2]).0;
        let crn    = ::util::imm_to_u32(arm.operands()[3].data());
        let crm    = ::util::imm_to_u32(arm.operands()[4].data());
        let opc2   = ::util::imm_to_u32(arm.operands()[5].data());

        self.cp15_set_reg(crn, opc1, rt, crm, opc2);

        None
    }
}
