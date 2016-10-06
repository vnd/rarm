use capstone::ffi::*;

/* p. 776 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_umlal(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_xmlal(insn, false)
    }
}
