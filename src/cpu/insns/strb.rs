use capstone::ffi::*;

/* p. 680 (imm)
   p. 682 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_strb(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_strx(insn, 1)
    }
}
