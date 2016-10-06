use capstone::ffi::*;
use util::{get_bit, get_bits};

impl ::cpu::core::CPU {
    pub unsafe fn exec_ldrhx(&mut self, insn: &Insn, sign_extend: bool) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }
        let raw: u32 = self.mem.read(insn.address as usize);
        let p = get_bit(raw, 24);
        let u = get_bit(raw, 23);
        let w = get_bit(raw, 21);
        let n = get_bits(raw, (16..19)) as u8;
        let t = get_bits(raw, (12..15)) as u8;
        let imm4h = get_bits(raw, (8..11));
        let imm4l = get_bits(raw, (0..3));
        let m = imm4l as u8; // reg form
        let index = match p {
            1 => true,
            0 => false,
            _ => { unreachable!() },
        };
        let add = match u {
            1 => true,
            0 => false,
            _ => { unreachable!() },
        };
        let mut wback = false;
        if p == 0 || w == 1 {
            wback = true;
            assert!(n != 15 && n != t);
        }
        assert!(t != 15);
        let val = match get_bit(raw, 22) {
            1 => imm4l + (imm4h << 4), // imm
            0 => self.get_reg(m), // reg, shift seems to not have any effect since shift_n is zero
            _ => unreachable!(),
        };

        let offset_addr = if add { self.get_reg(n) + val } else { self.get_reg(n) - val };
        let address = if index { offset_addr } else { self.get_reg(n) };
        if wback {
            self.set_reg(n, offset_addr);
        }
        let data = if sign_extend {
            ::arith::sign_extend_u16(self.mem.read_halfword(address as usize))
        } else { self.mem.read_halfword(address as usize) as u32 };
        self.set_reg(t, data);

        //LDRH seems to be decoded incorrectly by capstone:
        //https://github.com/aquynh/capstone/issues/695
        //self.exec_ldrx(insn, 2)
        None
    }
}
