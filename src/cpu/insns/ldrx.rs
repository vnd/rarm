use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 418 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrx(&mut self, insn: &Insn, bytes: u32, sign_extend: bool) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }
        let raw: u32 = self.mem.read(insn.address as usize);
        let index = ::util::get_bit(raw, 24);

        ::util::assert_shift(&arm.operands()[0..1]);
        ::util::check_subtracted(&arm.operands(), insn);
        assert!(arm.operands().len() == 2 || arm.operands().len() == 3);
        if arm.operands().len() == 3 {
            assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
            assert!(index == 0);
        } else {
            assert!(index == 1);
        }
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_MEM);
        assert!(false == arm.update_flags);

        let t = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());

        let offset_addr = match index {
            0 => {
                let imm = ::util::imm_to_u32(arm.operands()[2].data());
                self.op_value(&arm.operands()[1]).0 + imm
            },
            1 => self.op_value(&arm.operands()[1]).0,
            _ => { assert!(false); 0 },
        };

        let address = match index {
            1 => offset_addr,
            0 => self.get_reg(n),
            _ => { assert!(false); 0 },
        };

        let value = match bytes {
            1 => self.mem.read_byte(address as usize) as u32,
            2 => self.mem.read_halfword(address as usize) as u32,
            _ => { assert!(false); 0 },
        };

        if t == 15 {
            assert!(false); // t != 15 for imm ldrb at least
            return Some(value);
        } else {
            if sign_extend {
                self.set_reg(t, ::arith::sign_extend_u32(value));
            } else {
                self.set_reg(t, value);
            }
        }

        if true == arm.writeback {
            self.set_reg(n, offset_addr);
        }

        None
    }
}
