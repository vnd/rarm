use capstone::ffi::*;
use capstone::ffi::detail::*;

impl ::cpu::core::CPU {
    pub unsafe fn exec_strx(&mut self, insn: &Insn, bytes: u32) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        let raw: u32 = self.mem.read(insn.address as usize);
        let index = ::util::get_bit(raw, 24);
        assert!(arm.operands().len() == 2 || arm.operands().len() == 3);
        if arm.operands().len() == 3 {
            assert!(index == 0);
        } else {
            assert!(index == 1);
        }
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_MEM);
        ::util::assert_shift(&arm.operands()[0..1]);
        ::util::check_subtracted(arm.operands(), insn);

        let t = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let offset_addr = match index {
            0 => {
                let val = match arm.operands()[2].ty {
                    ARMOpType::ARM_OP_IMM => ::util::imm_to_u32(arm.operands()[2].data()),
                    ARMOpType::ARM_OP_REG => self.op_value(&arm.operands()[2]).0,
                    _ => unreachable!(),
                };
                self.op_value(&arm.operands()[1]).0 + val
            },
            1 => self.op_value(&arm.operands()[1]).0,
            _ => { assert!(false); 0 },
        };

        let address = match index {
            1 => offset_addr,
            0 => self.get_reg(n),
            _ => { assert!(false); 0 },
        };

        let rt = self.get_reg(t);
        match bytes {
            1 => self.mem.write_byte(address as usize, ::util::get_bits(rt, 0..7) as usize),
            2 => self.mem.write_halfword(address as usize, ::util::get_bits(rt, 0..15) as usize),
            _ => { assert!(false) },
        };

        let writeback = ::util::get_bit(raw, 24) == 0 || ::util::get_bit(raw, 21) == 1;
        if writeback {
            self.set_reg(n, offset_addr);
        }

        None
    }
}
