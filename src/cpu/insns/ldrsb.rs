use capstone::ffi::*;

/* p. 450 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrsb(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_ldrx(insn, 1, true)
    }
}
