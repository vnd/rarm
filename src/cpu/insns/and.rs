use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;

/* p. 324 (imm)
   p. 326 (register)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_and(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands().len() == 3);
        assert!(false == arm.writeback);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let (val, carry) = match arm.operands()[2].ty {
            ARMOpType::ARM_OP_REG => {
                self.op_value(&arm.operands()[2])
            },
            ARMOpType::ARM_OP_IMM => {
                ::util::assert_shift(&arm.operands());
                let raw: u32 = self.mem.read(insn.address as usize);
                expand_imm_c(::util::get_bits(raw, 0..11), self.get_carry())
            },
            _ => { assert!(false); (0, 0) }
        };

        let result = self.get_reg(n) & val;

        if d == 15 {
            assert!(false == arm.update_flags);
            return Some(result);
        }

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            // CPSR.V unchanged
        }
        None
    }
}
