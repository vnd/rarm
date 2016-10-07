use std::mem::transmute;
use capstone::ffi::detail::*;

/* boilerplate for parsing capstone ops: implementation of op_value() method */

impl ::cpu::core::CPU {
    fn reg_from_shift(&self, op: &ARMOp) -> u32 {
        unsafe { self.get_reg(::util::cs_reg_num(transmute(op.shift_value))) }
    }

    fn op_value_reg_imm(&self, op: &ARMOp) -> (u32, u32) {
        assert!(!op.subtracted);
        let value = match op.data() {
            ARMOpData::Reg(r) => self.get_reg(::util::cs_reg_num(r)) as u32,
            ARMOpData::Imm(i) => i,
            _ => { unreachable!() },
        };

        let carry = self.get_cpsr_bit(::cpu::cpsr::CPSR_C);
        let shifter = unsafe { op.shifter() };
        match shifter {
            ARMShifter::ARM_SFT_LSL_REG =>
                ::arith::shift_c(value, ARMShifter::ARM_SFT_LSL, self.reg_from_shift(op), carry),
            ARMShifter::ARM_SFT_LSR_REG =>
                ::arith::shift_c(value, ARMShifter::ARM_SFT_LSR, self.reg_from_shift(op), carry),
            ARMShifter::ARM_SFT_ROR_REG =>
                ::arith::shift_c(value, ARMShifter::ARM_SFT_ROR, self.reg_from_shift(op), carry),
            ARMShifter::ARM_SFT_ASR_REG =>
                ::arith::shift_c(value, ARMShifter::ARM_SFT_ASR, self.reg_from_shift(op), carry),
            _ => ::arith::shift_c(value, shifter, op.shift_value, carry)
        }
    }

    fn op_value_mem(&self, op: &ARMOp, override_scale: bool) -> (u32, u32) {
        let shifter = unsafe { op.shifter() };
        let mut carry = self.get_cpsr_bit(::cpu::cpsr::CPSR_C);
        match op.data() {
            ARMOpData::Mem(m) => {
                let reg: ARMReg = unsafe { transmute(m.base) };
                let mut address = self.get_reg(::util::cs_reg_num(reg));
                assert!(m.scale == 1 || m.scale == -1);
                if m.index > 0 {
                    assert!(m.disp == 0);
                    let index = unsafe { self.get_reg(::util::cs_reg_num(transmute(m.index))) };
                    let (shifted, new_carry) = match shifter {
                        ARMShifter::ARM_SFT_LSL |
                        ARMShifter::ARM_SFT_LSR => ::arith::shift_c(index, shifter, op.shift_value, carry),
                        ARMShifter::ARM_SFT_INVALID => (index, 0),
                        _ => unreachable!(),
                    };
                    carry = new_carry;
                    if m.scale == 1 || override_scale { // bug in capstone for strd and ldrd
                        address += shifted;
                    } else {
                        address -= shifted;
                    }
                } else {
                    assert!(op.shift_type == 0);
                    if m.disp < 0 {
                        address -= -m.disp as u32;
                    } else {
                        address += m.disp as u32;
                    }
                }
                (address, carry)
            },
            _ => { unreachable!() }
        }
    }

    pub fn _op_value(&self, op: &ARMOp, override_scale: bool) -> (u32, u32) {
        match op.ty {
            ARMOpType::ARM_OP_REG |
            ARMOpType::ARM_OP_IMM => self.op_value_reg_imm(op),
            ARMOpType::ARM_OP_MEM => self.op_value_mem(op, override_scale),
            _ => unreachable!(),
        }
    }

    pub fn op_value(&self, op: &ARMOp) -> (u32, u32) {
        self._op_value(op, false)
    }
}
