impl ::cpu::core::CPU {
    // p. 1216
    // http://unix.stackexchange.com/questions/231116/what-happens-after-a-page-fault
    pub fn do_page_fault(&mut self) -> u32 {
        println!("page fault at 0x{:x}, va is 0x{:x}", self.get_pc(), unsafe {::memory::dfar});
        unsafe { ::memory::fault = false };
        let cpsr = self.cpsr;

        // The processor switches to abort mode (one of the kernel-level privileged modes).
        self.set_cpsr_bits(::cpu::cpsr::CPSR_M, ::cpu::core::ProcessorMode::Abt as u32);

        // The lr register is set to the program counter at the time of the fault
        let pc = self.get_reg(15);
        self.set_reg(14, pc);

        // and the spsr register is set to the program status register () at the time of the fault.
        self.set_spsr(cpsr);
        
        self.set_cpsr_bit(::cpu::cpsr::CPSR_I, 1);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_A, 1);

        // The execution jumps to the abort vector, one of the exception vectors.
        0xFFFF0010 as u32
    }
}
