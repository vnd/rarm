use capstone::{Handle};
use capstone::ffi::*;
use memory::Memory;
use ::cpu::cpsr::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProcessorMode {
    Usr = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Svc = 0b10011,
    Abt = 0b10111,
    Und = 0b11011,
    Sys = 0b11111
}

const QEMU_SCTLR: u32 = 0xc50078;

pub struct CPU {
    pub regs: [[u32; 16]; 32],
    pub cpsr: u32,
    pub spsr: [u32; 32],
    pub mode: ProcessorMode,
    pub mem: Memory,
    pub bp: u32,
    pub ignore_bp: u32,
    pub bpc: u64,
    pub verbose: bool,
    pub dacr: u32,
    pub sctlr: u32,
    pub thread_id: u32,
    pub steps: u32,
    pub insn_counter: u64,
}

impl CPU {
    pub fn new(mem: Memory, entry_addr: u32) -> CPU {
        assert!(entry_addr % 4 == 0);

        let mut cpu = CPU {
            mode: ProcessorMode::Svc,
            regs: [[0; 16]; 32],
            cpsr: 0,
            spsr: [0; 32],
            mem: mem,
            bp: 0,
            ignore_bp: 0,
            bpc: 0,
            verbose: false,
            dacr: 0,
            sctlr: QEMU_SCTLR,
            thread_id: 0,
            steps: 0,
            insn_counter: 0,
        };
        cpu.regs[ProcessorMode::Usr as usize][15] = entry_addr;
        cpu
    }

    // p. 1144
    pub fn register_banking(&self, reg: u8) -> ProcessorMode {
        match reg {
            0  ... 7  => ProcessorMode::Usr,
            8  ... 12 => if self.mode == ProcessorMode::Fiq { self.mode } else { ProcessorMode::Usr },
            13 ... 14 => if self.mode == ProcessorMode::Sys { ProcessorMode::Usr } else { self.mode },
            15        => ProcessorMode::Usr,
            _         => unreachable!(),
        }
    }

    pub fn get_reg(&self, reg: u8) -> u32 {
        assert!(reg < 16);

        if reg == 15 {
            /* When executing an ARM instruction, PC reads as the address
               of the current instruction plus 8. (c) */
            return self.get_pc() + 8;
        }
        let mode = self.register_banking(reg);
        self.regs[mode as usize][reg as usize]
    }

    pub fn get_mode_reg(&self, mode: ProcessorMode, reg: u8) -> u32 {
        assert!(reg < 16);

        if reg == 15 {
            /* When executing an ARM instruction, PC reads as the address
               of the current instruction plus 8. (c) */
            return self.get_pc() + 8;
        }
        self.regs[mode as usize][reg as usize]
    }

    pub fn get_pc(&self) -> u32 {
        let mode = self.register_banking(15);
        self.regs[mode as usize][15]
    }

    pub fn set_reg(&mut self, reg: u8, val: u32) {
        let mode = self.register_banking(reg);
        self.regs[mode as usize][reg as usize] = val;
    }

    pub fn set_mode_reg(&mut self, mode: ProcessorMode, reg: u8, val: u32) {
        self.regs[mode as usize][reg as usize] = val;
    }

    pub fn set_pc(&mut self, pc: u32) {
        let mode = self.register_banking(15);
        self.regs[mode as usize][15] = pc;
    }

    pub unsafe fn start(&mut self, capstone: Handle, verbose: bool) {
        self.set_cpsr_bits(CPSR_M, ProcessorMode::Svc as u32);
        self.set_cpsr_bit(CPSR_I, 1);
        self.set_cpsr_bit(CPSR_F, 1);
        self.set_cpsr_bit(CPSR_A, 1);
        self.verbose = verbose;
        let mut regs_clone = self.regs;
        loop {
            if ::memory::fault {
                /* Current architecture does not allow to process page fault when it occurs.
                   The only easy possibility is after the faulty instruction is executed,
                   therefore restore registers to the previous state.
                   Exceptions in Rust (or better architecture) would allow to properly handle this. */
                self.regs = regs_clone;
                let fault_handler = self.do_page_fault();
                self.set_pc(fault_handler);
                ::memory::fault = false;
            } else {
                regs_clone = self.regs;
                let mut raw_insn: [u8; 4] = [0; 4];
                let raw_insn_u32: u32 = self.mem.read(self.get_pc() as usize); // may also fault
                if ::memory::fault {
                    continue;
                }
                ::util::u32_to_bytes(raw_insn_u32, &mut raw_insn);
                capstone.walk_insts(&raw_insn, self.get_pc() as u64, |insn: &Insn| {
                    let mut steps_done = false;
                    if self.steps > 0 {
                        self.steps -= 1;
                        if self.steps == 0 {
                            steps_done = true;
                        }
                    }

                    if steps_done || (self.get_pc() == self.bp && self.ignore_bp == 0) {
                        self.do_breakpoint(insn);
                    }

                    if self.get_pc() == self.bp && self.ignore_bp > 0 {
                        self.ignore_bp -= 1;
                    }

                    if self.verbose {
                        ::util::print_insn_mnemonic(insn, true);
                    }

                    let next = match self.execute_insn(insn) {
                        Some(addr) => addr,
                        None => self.get_pc() + 4,
                    };
                    self.set_pc(next);
                }).unwrap();
            }
            self.insn_counter += 1;

            if self.insn_counter == self.bpc {
                self.steps = 1;
            }

            if self.insn_counter % 100 == 0 {
                let (timer1_int, _) = ::timer::timer_tick(self.verbose);

                if timer1_int {
                    let irq_handler = self.do_irq(::cpu::interrupt::TIMER1_IRQ);
                    if irq_handler != None {
                        self.set_pc(irq_handler.unwrap());
                    }
                } else if ::uart::tx_irq || ::uart::rx_irq {
                    match self.do_irq(::cpu::interrupt::UART_IRQ) {
                        Some(handler) => self.set_pc(handler),
                        None => {},
                    }
                }
            }
        }
    }
}

impl Drop for CPU {
    fn drop(&mut self) {
        self.dump_state();
        self.mem.dump();
    }
}
