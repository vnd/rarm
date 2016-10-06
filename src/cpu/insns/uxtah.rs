use capstone::ffi::*;

// p. 810
impl ::cpu::core::CPU {
    pub unsafe fn exec_uxtah(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_uxtax(insn, 2)
    }
}
