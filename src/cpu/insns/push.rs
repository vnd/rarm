use capstone::ffi::*;
use capstone::ffi::detail::*;
use std::mem::transmute;

/* p. 539
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_push(&mut self, insn: &Insn) -> Option<u32> {
        let arm: *mut ARMDetail = transmute(&((*(insn.detail)).arch_data));
        
        for i in (1..(*arm).op_count+1).rev() {
            (*arm).operands[i as usize] = (*arm).operands[i as usize - 1].clone();
        }
        (*arm).operands[0].data[0] = transmute(ARMReg::ARM_REG_SP as u64);
        (*arm).writeback = true;
        (*arm).op_count += 1;
        self.exec_stmdb(insn)
    }
}
