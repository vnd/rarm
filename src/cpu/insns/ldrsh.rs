use capstone::ffi::*;

/* p. 458 (imm)
   p. xxx (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrsh(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_ldrhx(insn, true)
    }
}
