use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 674 (imm)

*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_str(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        //TODO str implementation looks somewhat fishy with regard to index/post/pre forms

        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_MEM);
        ::util::assert_shift(&arm.operands()[0..1]);

        let val = self.op_value(&arm.operands()[0]).0;
        let base_reg = ::util::reg_num(arm.operands()[1].data());
        let address = self.op_value(&arm.operands()[1]).0;
        self.mem.write(address as usize, val as usize);

        if arm.writeback {
            // 3 operands hopefully means post-indexed form
            if arm.operands().len() == 3 {
                assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
                let base_val = self.get_reg(base_reg);
                self.set_reg(base_reg, base_val + ::util::imm_to_u32(arm.operands()[2].data()));
            } else if arm.operands().len() == 2 {
                self.set_reg(base_reg, address);
            } else {
                assert!(false);
            }
        }

        None
    }
}
