use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 398
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldm(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(arm.operands());
        for op in arm.operands() {
            assert!(op.ty == ARMOpType::ARM_OP_REG);
        }
        assert!(!arm.update_flags);

        /* http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0204j/Cihcadda.html {^} */
        let mut copy_spsr = false;

        let mut explicit_mode = if insn.op_str().unwrap().ends_with("} ^") {
            Some(::cpu::core::ProcessorMode::Usr)
        } else { None };

        if explicit_mode != None {
            for i in 1..arm.operands().len() {
                if ::util::reg_num(arm.operands()[i].data()) == 15 {
                    copy_spsr = true;
                    explicit_mode = None;
                }
            }
        }
        assert!(explicit_mode == None);

        let n = ::util::reg_num(arm.operands()[0].data());
        let mut address = self.op_value(&arm.operands()[0]).0;
        let mut registers = 0;
        let mut ret: Option<u32> = None;
        for i in 1..arm.operands().len() {
            let r = ::util::reg_num(arm.operands()[i].data());
            let value = self.mem.read(address as usize);
            if r != 15 {
                self.set_reg(r, value);
            } else {
                ret = Some(value);
            }
            address += 4;
            registers += 1;
        }

        if arm.writeback {
            let val = self.get_reg(n);
            self.set_reg(n, val + 4 * registers);
        }

        if copy_spsr {
            let spsr = self.get_spsr();
            self.set_cpsr(spsr);
        }

        ret
    }
}
