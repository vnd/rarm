use capstone::ffi::*;

impl ::cpu::core::CPU {
    pub unsafe fn do_nothing(&mut self, _: &Insn) -> Option<u32> {
        None
    }
}
