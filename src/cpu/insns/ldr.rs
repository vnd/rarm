use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 408 (immediate)
   p. 410 (literal, used at all?)
   http://www.heyrick.co.uk/armwiki/LDR
   I - Register (set) or Immediate (unset)
   P - Pre-indexed (set) or Post-indexed (unset)
   U - Offset added to base (set) or subtracted from base (unset)
   B - Unsigned byte (set) or word (unset) access
   W - Depends on the P bit:
       [P = 1] - the calculated address will be written back if W set.
       [P = 0] - the access is treated as a User mode access if W set (has no effect in User mode).
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldr(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands()[0..1]);
        ::util::check_subtracted(arm.operands(), insn);

        let raw: u32 = self.mem.read(insn.address as usize);
        let index = ::util::get_bit(raw, 24);
        let add   = ::util::get_bit(raw, 23);
        // https://github.com/aquynh/capstone/issues/740
        let writeback = ::util::get_bit(raw, 24) == 0 || ::util::get_bit(raw, 21) == 1;
        let t = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());

        let ops = arm.operands().len();
        assert!(ops == 2 || ops == 3);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_MEM);
        assert!(!arm.update_flags);
        let imm = if ops == 3 {
            assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
            ::util::imm_to_u32(arm.operands()[2].data())
        } else { 0 };

        let offset_addr = match add {
            1 => self.op_value(&arm.operands()[1]).0 + imm,
            0 => self.op_value(&arm.operands()[1]).0 - imm,
            _ => unreachable!(),
        };

        let address = match index {
            1 => offset_addr,
            0 => self.get_reg(n),
            _ => unreachable!(),
        };

        if writeback {
            self.set_reg(n, offset_addr);
        }

        let data = self.mem.read(address as usize);
        if t == 15 {
            return Some(data);
        } else {
            self.set_reg(t, data);
        }

        None
    }
}
