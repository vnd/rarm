use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 468 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_lsl(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_lsx(insn, ARMShifter::ARM_SFT_LSL)
    }
}
