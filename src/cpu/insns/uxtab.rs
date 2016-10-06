use capstone::ffi::*;

// p. 806
impl ::cpu::core::CPU {
    pub unsafe fn exec_uxtab(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_uxtax(insn, 1)
    }
}
