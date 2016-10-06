use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 474 (reg)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_lsr(&mut self, insn: &Insn) -> Option<u32> {
        self.exec_lsx(insn, ARMShifter::ARM_SFT_LSR)
    }
}
