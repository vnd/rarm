use capstone::ffi::*;

/* p. 812
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_uxtb(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_uxtx(insn, 1)
    }
}
