use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::option::Option;
use util::{get_bit, get_bits, set_bit, set_bits};

pub static mut dfsr: u32 = 0;
pub static mut dfar: u32 = 0;
pub static mut fault: bool = false;

pub type ReadHookFn = &'static Fn(usize, usize) -> u32;
pub type WriteHookFn = &'static Fn(usize, usize, usize);

#[allow(dead_code)]
pub struct MemoryRegion {
    name: &'static str,
    base: usize,
    size: usize,
    pub mem: Vec<u8>,
    read_fn: Option<ReadHookFn>,
    write_fn: Option<WriteHookFn>
}

pub struct Memory {
    regions: Vec<MemoryRegion>,
    watchpoint: usize,
    mmu_on: bool,
    ttbr0: u32,
    ttbr1: u32,
    ttbcr: u32,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            regions: Vec::new(),
            watchpoint: 0,
            mmu_on: false,
            ttbr0: 0,
            ttbr1: 0,
            ttbcr: 0,
        }
    }

    pub fn add_region(&mut self, name: &'static str, base: usize, size: usize, read_fn: Option<ReadHookFn>, write_fn: Option<WriteHookFn>) {
        self.regions.push(MemoryRegion::new(name, base, size, read_fn, write_fn));
    }

    pub fn add_watchpoint(&mut self, address: u32) {
        self.watchpoint = address as usize;
    }

    // short translation table format: p. 1326
    pub fn do_mmu(&self, va: usize, write: bool) -> (usize, bool) {
        if false == self.mmu_on {
            return (va, true);
        }
        let n = get_bits(self.ttbcr, 0..2);
        assert!(n == 0); // if not 0 then TTBR1 comes into play
        let x = 14 - n;
        let page_table = (get_bits(self.ttbr0, x..31) as usize) << x;
        let index = get_bits(va as u32, 20..31) as usize;
        let entry_addr = page_table + 4 * index;
        let entry = self.regions[0].read(entry_addr, va);
        let domain = get_bits(entry, 5..8);
        if get_bits(entry, 0..1) == 0b10 && get_bit(entry, 18) == 0 {
            // sections, p. 1335
            assert!(domain == 0);
            let writeable = get_bit(entry, 15) == 0; // AP[2]
            // TEX: 12..14, C: 3, B: 2 Seem to be not relevant for emulation
            let mut pa: u32 = va as u32;
            set_bits(&mut pa, 20..31, get_bits(entry, 20..31));

            return (pa as usize, writeable)
        } else if get_bits(entry, 0..1) == 0b01 {
            // small pages, p. 1337
            // 'entry' is 'first-level descriptor' in the above page
            let page_table_base_address = get_bits(entry, 10..31);
            let l2_table_index = get_bits(va as u32, 12..19);
            let second_level_descriptor_address = (page_table_base_address << 10) + (l2_table_index << 2);
            let second_level_descriptor = self.regions[0].read(second_level_descriptor_address as usize, va);
            if get_bit(second_level_descriptor, 1) != 1 {
                unsafe {
                    fault = true;
                    dfar = va as u32;
                    dfsr = 0x17; // second level translation fault
                    set_bit(&mut dfsr, 11, if write { 1 } else { 0 }); // WnR bit
                }
                return (0, true);
            }
            let page_index = get_bits(va as u32, 0..11);
            let small_page_base_address = get_bits(second_level_descriptor, 12..31);
            let pa = page_index + (small_page_base_address << 12);
            let writeable = get_bit(entry, 9) == 0; // AP[2]
            return (pa as usize, writeable)
        } else {
            // 0 means fault entry
            println!("lower bits of pte are 0x{:x}, va is 0x{:x}", get_bits(entry, 0..1), va);
            unsafe {
                fault = true;
                dfar = va as u32;
                dfsr = 0x5; // first level translation fault
                set_bit(&mut dfsr, 11, if write { 1 } else { 0 }); // WnR bit
            }
            (0, true)
        }
    }

    fn find_region(&self, va: usize) -> (&MemoryRegion, usize, bool) {
        let (pa, write) = self.do_mmu(va, false);
        for region in &self.regions {
            if pa >= region.base && pa < region.base + region.size {
                return (region, pa, write)
            }
        }
        panic!("Region not found for address 0x{:x} (0x{:x})", va, pa);
    }

    fn find_region_mut(&mut self, va: usize) -> (&mut MemoryRegion, usize, bool) {
		let (pa, write) = self.do_mmu(va, true);
        for region in &mut self.regions {
            if pa >= region.base && pa < region.base + region.size {
                return (region, pa, write)
            }
        }
        panic!("Region not found for address 0x{:x} (0x{:x})", va, pa);
    }

    pub fn read(&self, addr: usize) -> u32 {
        assert!(addr <= ::std::u32::MAX as usize);
		let (region, pa, _) = self.find_region(addr);
        region.read(pa, addr)
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        assert!(addr <= ::std::u32::MAX as usize);
		let (region, pa, _) = self.find_region(addr);
        region.read_byte(pa, addr)
    }

    pub fn read_halfword(&self, addr: usize) -> u16 {
        assert!(addr <= ::std::u32::MAX as usize);
		let (region, pa, _) = self.find_region(addr);
        region.read_halfword(pa, addr)
    }

    pub fn write(&mut self, addr: usize, value: usize) {
        let wp = self.watchpoint;
        if wp >= addr && wp < addr + 4 {
            println!("write watchpoint: value is {:x}, address {:x}", value, addr);
        }
		let (region, pa, writeable) = self.find_region_mut(addr);
		assert!(writeable);
        region.write(pa, addr, value)
    }

    pub fn write_byte(&mut self, addr: usize, value: usize) {
        let wp = self.watchpoint;
        if wp == addr {
            println!("write_byte watchpoint: value is {:x}, address {:x}", value, addr);
        }
		let (region, pa, writeable) = self.find_region_mut(addr);
		assert!(writeable);
        region.write_byte(pa, addr, value)
    }

    pub fn write_halfword(&mut self, addr: usize, value: usize) {
        let wp = self.watchpoint;
        if wp >= addr && wp < addr + 2 {
            println!("write_halfword watchpoint: value is {:x}, address {:x}", value, addr);
        }
		let (region, pa, writeable) = self.find_region_mut(addr);
		assert!(writeable);
        region.write_halfword(pa, addr, value)
    }

    pub fn load(&mut self, addr: usize, file: &'static str) {
		let (region, pa, _) = self.find_region_mut(addr);
        region.load(pa, file)
    }

    pub fn dump(&self) {
        self.regions[0].dump();
    }

    pub fn mmu_on(&mut self, on: bool) {
        self.mmu_on = on;
    }

	pub fn set_ttbcr(&mut self, ttbcr: u32) {
        assert!(get_bit(ttbcr, 31) == 0);
        assert!(get_bits(ttbcr, 0..2) == 0);

        self.ttbcr = ttbcr;
        println!("TTBCR is set to 0x{:x}", ttbcr);
	}

	pub fn set_ttbr0(&mut self, ttbr0: u32) {
        self.ttbr0 = ttbr0;
        println!("TTBR0 is set to 0x{:x}", ttbr0);
	}

	pub fn set_ttbr1(&mut self, ttbr1: u32) {
        self.ttbr1 = ttbr1;
        println!("TTBR1 is set to 0x{:x}", ttbr1);
	}
}

impl MemoryRegion {
    pub fn new(name: &'static str, base: usize, size: usize, read_fn: Option<ReadHookFn>, write_fn: Option<WriteHookFn>) -> MemoryRegion {
        let mut memory = MemoryRegion {
            name: name,
            base: base,
            size: size,
            mem: Vec::with_capacity(size),
            read_fn: read_fn,
            write_fn: write_fn
        };
        memory.mem.resize(size, 0);
        memory
    }

    pub fn write(&mut self, pa: usize, va: usize, value: usize) {
        assert!(value <= ::std::u32::MAX as usize);
        let addr = pa - self.base;
        ::util::u32_to_bytes(value as u32, &mut self.mem[addr..addr+4]);
        match self.write_fn {
            Some(write_fn) => write_fn(addr, va, value),
            None => {}
        }
    }

    pub fn write_byte(&mut self, pa: usize, va: usize, value: usize) {
        assert!(value <= ::std::u8::MAX as usize);
        let addr = pa - self.base;
        self.mem[addr] = value as u8;
        match self.write_fn {
            Some(write_fn) => write_fn(addr, va, value),
            None => {}
        }
    }

    pub fn write_halfword(&mut self, pa: usize, va: usize, value: usize) {
        assert!(value <= ::std::u16::MAX as usize);
        let addr = pa - self.base;
        ::util::u16_to_bytes(value as u16, &mut self.mem[addr..addr+2]);
        match self.write_fn {
            Some(write_fn) => write_fn(addr, va, value),
            None => {}
        }
    }

    pub fn read(&self, pa: usize, va: usize) -> u32 {
        let addr = pa - self.base;
        match self.read_fn {
            Some(read_fn) => read_fn(addr, va),
            None => ::util::bytes_to_u32(&self.mem[addr..addr+4])
        }
    }

    pub fn read_byte(&self, pa: usize, va: usize) -> u8 {
        let addr = pa - self.base;
        
        match self.read_fn {
            Some(read_fn) => read_fn(addr, va) as u8,
            None => self.mem[addr] 
        }
    }

    pub fn read_halfword(&self, pa: usize, va: usize) -> u16 {
        let addr = pa - self.base;
        match self.read_fn {
            Some(read_fn) => {
                println!("read_halfword: 0x{:x} 0x{:x}", addr, va);
                read_fn(addr, va) as u16
            },
            None => ::util::bytes_to_u16(&self.mem[addr..addr+2])
        }
    }

    pub fn load(&mut self, pa: usize, file: &'static str) {
        let addr = pa - self.base;
        let mut bytes: Vec<u8> = Vec::new();
        for i in 0..File::open(file).unwrap().read_to_end(&mut bytes).unwrap() {
            self.mem[addr + i] = bytes[i];
        }
    }

    pub fn dump(&self) {
       let mut f = File::create("mem.dump").unwrap();
       f.write_all(&self.mem).unwrap();
    }
}
