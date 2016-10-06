use capstone::ffi::*;

/* p. 816
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_uxth(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_uxtx(insn, 2)
    }
}
