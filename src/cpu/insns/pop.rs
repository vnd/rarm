use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 536
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_pop(&mut self, insn: &Insn) -> Option<u32> {
        let arm = &((*(insn.detail)).arch_data) as *const [u64; 185] as *mut ARMDetail;
        for i in (1..(*arm).op_count+1).rev() {
            (*arm).operands[i as usize] = (*arm).operands[i as usize - 1].clone();
        }
        (*arm).operands[0].data[0] = ARMReg::ARM_REG_SP as u64;
        (*arm).writeback = true;
        (*arm).op_count += 1;
        self.exec_ldm(insn)
    }
}
