use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 721 and p. 1211 (TakeSVCException) */
impl ::cpu::core::CPU {
    pub unsafe fn exec_svc(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        assert!(arm.operands().len() == 1);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_IMM);
        assert!(!arm.update_flags);
        assert!(!arm.writeback);

        println!("syscall {}", self.get_reg(7));
        // p. 1211
        let new_lr_value = self.get_pc() + 4;
        let new_spsr_value = self.cpsr;
        self.set_cpsr_bits(::cpu::cpsr::CPSR_M, ::cpu::core::ProcessorMode::Svc as u32);
        self.set_spsr(new_spsr_value);
        self.set_reg(14, new_lr_value);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_I, 1);

        Some(0xFFFF0008 as u32)
    }
}
