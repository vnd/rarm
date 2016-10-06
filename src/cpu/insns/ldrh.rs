use capstone::ffi::*;

/* p. 442 (imm)
   p. 446 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrh(&mut self, insn: &Insn) -> Option<u32> {
		self.exec_ldrhx(insn, false)
    }
}
