use capstone::ffi::*;
use capstone::ffi::detail::*;
use arith::expand_imm_c;
use util::get_bits;

/* p. 484 (imm)
   p. 489 (register)
*/


fn mov_imm(raw: u32, carry: u32) -> (u32, u32) {
    match get_bits(raw, (21..23)) {
        0b101 => expand_imm_c(get_bits(raw, 0..11), carry),
        0 => {
            let imm4 = get_bits(raw, (16..19));
            let imm12 = get_bits(raw, (0..11));
            (imm12 + (imm4 << 12), 0)
        },
        _ => { assert!(false); (0, 0) },
    }
}

impl ::cpu::core::CPU {
    pub unsafe fn exec_mov(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        // https://github.com/aquynh/capstone/issues/690
        let lsr = insn.mnemonic().unwrap()[..3].to_string() == "lsr";
        if arm.operands().len() == 3 && lsr {
            return self.exec_lsr(insn);
        }

        // https://github.com/aquynh/capstone/issues/690
        let lsl = insn.mnemonic().unwrap()[..3].to_string() == "lsl";
        if arm.operands().len() == 3 && lsl {
            return self.exec_lsl(insn);
        }

        // https://github.com/aquynh/capstone/issues/690
        let asr = insn.mnemonic().unwrap()[..3].to_string() == "asr";
        if arm.operands().len() == 3 && asr {
            return self.exec_asr(insn);
        }

        if insn.mnemonic().unwrap()[..3].to_string() == "rrx" {
            return self.exec_rrx(insn);
        }

        let ror = insn.mnemonic().unwrap()[..3].to_string() == "ror";
        if ror {
            assert!(arm.operands[1].shift_type == 4);
        }

        let mnem = insn.mnemonic().unwrap()[..3].to_string();
        if mnem != "mov" && !lsr && !lsl && !asr && !ror{
            ::util::dump_insn(insn, true);
            println!("{} pretending to be mov", mnem); // capstone, oh capstone
            assert!(false);
        }

        assert!(arm.operands().len() == 2);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(false == arm.writeback);
        ::util::assert_shift(&arm.operands()[0..1]);

        let raw: u32 = self.mem.read(insn.address as usize);
        let (result, carry) = match arm.operands()[1].ty {
            ARMOpType::ARM_OP_IMM => mov_imm(raw, self.get_carry()),
            ARMOpType::ARM_OP_REG => self.op_value(&arm.operands()[1]),
            _ => { assert!(false); (0, 0) },
        };

        let d = ::util::reg_num(arm.operands()[0].data());
        if d == 15 {
            if arm.update_flags {
                let spsr = self.get_spsr();
                // p. 2012
                self.set_cpsr(spsr);
            }
            return Some(result);
        }

        self.set_reg(d, result);

        if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            if asr || lsr || lsl || arm.operands()[1].ty == ARMOpType::ARM_OP_IMM {
                self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            }
            // APSR.V unchanged
        }
        None
    }
}
