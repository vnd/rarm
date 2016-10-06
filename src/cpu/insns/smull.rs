use capstone::ffi::*;

/* p. 646 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_smull(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_xmull(insn, true)
    }
}
