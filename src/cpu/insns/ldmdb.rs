use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 402 */
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldmdb(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        for op in arm.operands() {
            assert!(op.ty == ARMOpType::ARM_OP_REG);
        }
        assert!(!arm.update_flags);

        /* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0204j/Cihcadda.html {^}
         * Otherwise, data is transferred into or out of the User mode registers
         * instead of the current mode registers. (c) */
        let explicit_mode = if insn.op_str().unwrap().ends_with("} ^") {
            Some(::cpu::core::ProcessorMode::Usr)
        } else { None };

        let len = arm.operands().len() - 1; // w/o base register
        let mut address = self.op_value(&arm.operands()[0]).0 - (4 * len) as u32;
        let n = ::util::reg_num(arm.operands()[0].data());
        for i in 1..(len + 1) {
            let r = ::util::reg_num(arm.operands()[i].data());
            assert!(r != 15);
            let value = self.mem.read(address as usize);
            match explicit_mode {
                Some(mode) => self.set_mode_reg(mode, r, value),
                None => self.set_reg(r, value),
            };
            address += 4;
        }

        if arm.writeback {
            let val = match explicit_mode {
                Some(mode) => self.get_mode_reg(mode, n) - (4 * len) as u32,
                None => self.get_reg(n) - (4 * len) as u32,
            };
            match explicit_mode {
                Some(mode) => self.set_mode_reg(mode, n, val),
                None => self.set_reg(n, val),
            };
        }

        None
    }
}
