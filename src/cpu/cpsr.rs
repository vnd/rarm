use std::ops::Range;
use std::mem::transmute;

// p. 1148
pub const CPSR_N: u32 = 31;             // negative
pub const CPSR_Z: u32 = 30;             // zero
pub const CPSR_C: u32 = 29;             // carry
pub const CPSR_V: u32 = 28;             // overflow
pub const CPSR_Q: u32 = 27;             // cummulative saturation
pub const CPSR_GE: Range<u32> = 16..19; // relevant for SIMD only?
pub const CPSR_E: u32 = 9;              // endianness
pub const CPSR_A: u32 = 8;              // async abort mask
pub const CPSR_I: u32 = 7;              // IRQ mask
pub const CPSR_F: u32 = 6;              // FIQ mask
pub const CPSR_T: u32 = 5;              // Thumb
pub const CPSR_M: Range<u32> = 0..4;    // mode

impl ::cpu::core::CPU {
    pub fn get_spsr(&self) -> u32 {
        self.spsr[self.mode as usize]
    }

    pub fn set_spsr(&mut self, spsr: u32) {
        self.spsr[self.mode as usize] = spsr;
    }

    pub fn get_cpsr_bit(&self, bit_num: u32) -> u32 {
        ::util::get_bit(self.cpsr, bit_num)
    }

    pub fn get_carry(&self) -> u32 {
        ::util::get_bit(self.cpsr, CPSR_C)
    }

    pub fn set_cpsr(&mut self, new_cpsr: u32) {
		let prev_mode = self.mode;
		self.mode = unsafe { transmute(::util::get_bits(new_cpsr, CPSR_M) as u8) };
		if self.mode != prev_mode {
			println!("Switched to {:#?} from {:#?} at 0x{:x}", self.mode, prev_mode, self.get_pc());
		}
        self.cpsr = new_cpsr;
    }

    pub fn set_cpsr_bit(&mut self, bit_num: u32, val: u32) {
        let mut cpsr = self.cpsr;
        ::util::set_bit(&mut cpsr, bit_num, val);
        self.set_cpsr(cpsr);
    }

    pub fn set_cpsr_bits(&mut self, range: Range<u32>, bits: u32) {
        let mut cpsr = self.cpsr;
        ::util::set_bits(&mut cpsr, range, bits);
        self.set_cpsr(cpsr);
    }
}
