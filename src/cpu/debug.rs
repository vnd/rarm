use std::io::{self, BufRead};
use capstone::ffi::*;

impl ::cpu::core::CPU {
    pub fn dump_state(&self) {
        println!("executed {} instructions", self.insn_counter);
        println!("r0\t\t0x{:x}", self.get_reg(0));
        println!("r1\t\t0x{:x}", self.get_reg(1));
        println!("r2\t\t0x{:x}", self.get_reg(2));
        println!("r3\t\t0x{:x}", self.get_reg(3));
        println!("r4\t\t0x{:x}", self.get_reg(4));
        println!("r5\t\t0x{:x}", self.get_reg(5));
        println!("r6\t\t0x{:x}", self.get_reg(6));
        println!("r7\t\t0x{:x}", self.get_reg(7));
        println!("r8\t\t0x{:x}", self.get_reg(8));
        println!("r9\t\t0x{:x}", self.get_reg(9));
        println!("r10\t\t0x{:x}", self.get_reg(10));
        println!("r11\t\t0x{:x}", self.get_reg(11));
        println!("r12\t\t0x{:x}", self.get_reg(12));
        println!("sp\t\t0x{:x}", self.get_reg(13));
        println!("lr\t\t0x{:x}", self.get_reg(14));
        println!("pc\t\t0x{:x}", self.get_pc());
        println!("cpsr\t\t0x{:x}", self.cpsr);
        println!("");
    }

    pub fn set_breakpoint(&mut self, bp: u32, ignore_bp: u32) {
        self.bp = bp;
        self.ignore_bp = ignore_bp;
    }

    pub fn set_count_breakpoint(&mut self, bpc: u64) {
        self.bpc = bpc;
    }

    // http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0204j/Cihcadda.html {^}
    pub fn assert_exception_return(&self, insn: &Insn) {
        assert!(!insn.op_str().unwrap().ends_with("} ^"));
    }

    pub fn do_breakpoint(&mut self, insn: &Insn) {
        println!("stopped at breakpoint/step");
        unsafe { ::util::dump_insn(insn, true); }
        self.dump_state();
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        if line.contains("x 0x") {
            let address = u32::from_str_radix(&line.as_str()[4..], 16).unwrap();
            let (pa, _) = self.mem.do_mmu(address as usize, false);
            println!("0x{:x} (0x{:x}): 0x{:x}", address, pa, self.mem.read(address as usize));
            self.steps = 1;
        } else if line.contains("0x") {
            self.bp = u32::from_str_radix(&line.as_str()[2..], 16).unwrap();
        }
        else if line == "v" {
            self.verbose = !self.verbose;
        } else {
            let stepi = line.parse::<u32>();
            if stepi.is_ok() {
                self.steps = stepi.unwrap();
            }
        }
    }
}
