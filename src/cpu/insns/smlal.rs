use capstone::ffi::*;

/* p. 624 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_smlal(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_xmlal(insn, true)
    }
}
