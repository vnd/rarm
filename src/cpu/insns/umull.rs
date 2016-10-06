use capstone::ffi::*;

/* p. 778 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_umull(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_xmull(insn, false)
    }
}
