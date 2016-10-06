use capstone::ffi::detail::*;

impl ::cpu::core::CPU {
    pub fn cond_passed(&self, cond: ARMCC) -> bool {
        match cond {
            // p. 288
            ARMCC::ARM_CC_AL => true,
            ARMCC::ARM_CC_LO => self.get_cpsr_bit(::cpu::cpsr::CPSR_C) == 0,
            ARMCC::ARM_CC_EQ => self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 1,
            ARMCC::ARM_CC_GE => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) ==
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_V),
            ARMCC::ARM_CC_GT => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) ==
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_V) && 
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 0,
            ARMCC::ARM_CC_LE => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) !=
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_V) || 
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 1,
            ARMCC::ARM_CC_HI => self.get_cpsr_bit(::cpu::cpsr::CPSR_C) == 1 &&
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 0,
            ARMCC::ARM_CC_HS => self.get_cpsr_bit(::cpu::cpsr::CPSR_C) == 1,
            ARMCC::ARM_CC_LS => self.get_cpsr_bit(::cpu::cpsr::CPSR_C) == 0 ||
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 1,
            ARMCC::ARM_CC_LT => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) !=
                                self.get_cpsr_bit(::cpu::cpsr::CPSR_V),
            ARMCC::ARM_CC_PL => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) == 0,
            ARMCC::ARM_CC_NE => self.get_cpsr_bit(::cpu::cpsr::CPSR_Z) == 0,
            ARMCC::ARM_CC_MI => self.get_cpsr_bit(::cpu::cpsr::CPSR_N) == 1,
            _ => {
                println!("{:#?} is not implemented", cond);
                unreachable!()
            }
        }
    }
}
