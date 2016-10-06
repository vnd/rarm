use capstone::ffi::*;

/* p. 418 (imm)
   p. 422 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrb(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_ldrx(insn, 1, false)
    }
}
