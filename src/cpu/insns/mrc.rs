use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 492 */

impl ::cpu::core::CPU {
    pub unsafe fn exec_mrc(&mut self, insn: &Insn) -> Option<u32> {
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
        assert!(false == arm.update_flags);
        assert!(false == arm.writeback);

        let coproc = ::util::imm_to_u32(arm.operands()[0].data());
        let opc1   = ::util::imm_to_u32(arm.operands()[1].data());
        let t      = ::util::reg_num(arm.operands()[2].data());
        let crn    = ::util::imm_to_u32(arm.operands()[3].data());
        let crm    = ::util::imm_to_u32(arm.operands()[4].data());
        let opc2   = ::util::imm_to_u32(arm.operands()[5].data());

        let cp15_reg = match coproc {
            14 => self.cp14_get_reg(crn, opc1, crm, opc2),
            15 => self.cp15_get_reg(crn, opc1, crm, opc2),
            _ => unreachable!(),
        };
        self.set_reg(t, cp15_reg);

/*
ee109f10 (hex)

Insn { address: 1610679248, size: 4, mnemonic: Some("mrc"), op_str: Some("p15, #0, sb, c0, c0, #0") }

ARMDetail { usermode: false, vector_size: 0, vector_data: 0, cps_mode: ARM_CPSMODE_INVALID, cps_flag: ARM_CPSFLAG_INVALID, cc: ARM_CC_AL, update_flags: false, writeback: false, mem_barrier: 0, op_count: 6, operands: [
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_PIMM, data: [15, 0], subtracted: false },
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_IMM, data: [0, 0], subtracted: false },
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_REG, data: [75, 0], subtracted: false },
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_CIMM, data: [0, 0], subtracted: false },
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_CIMM, data: [0, 0], subtracted: false },
ARMOp { vector_index: -1, shift_type: 0, value: 0, ty: ARM_OP_IMM, data: [0, 0], subtracted: false }] }
*/

        None
    }
}
