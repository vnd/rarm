use capstone::ffi::*;
use util::{get_bit, get_bits};

/* p. 700 (imm)
   p. 702 (reg)

*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_strh(&mut self, insn: &Insn) -> Option<u32> {
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
        let rt = self.get_reg(t);
        let val = match get_bit(raw, 22) {
            1 => imm4l + (imm4h << 4), // imm
            0 => self.get_reg(m), // reg
            _ => unreachable!(),
        };

        let offset_addr = if add { self.get_reg(n) + val } else { self.get_reg(n) - val };
        let address = if index { offset_addr } else { self.get_reg(n) };
        assert!(::util::get_bit(address, 0) == 0);
        self.mem.write_halfword(address as usize, ::util::get_bits(rt, 0..15) as usize);
        if wback {
            self.set_reg(n, offset_addr);
        }

        //self.exec_strx(insn, 2)
        None
    }
}
