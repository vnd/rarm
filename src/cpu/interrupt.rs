pub const TIMER1_IRQ: u32 = 34;
pub const TIMER2_IRQ: u32 = 35;
pub const UART_IRQ:   u32 = 37;
pub const SPURIOUS_IRQ: u32 = 1023;

#[derive(Debug, Clone)]
enum IrqStatus {
    Inactive,
    Pending,
    Active,
}

static mut irq_status: IrqStatus = IrqStatus::Inactive;

static mut icciar: u32 = SPURIOUS_IRQ;
static mut iccicr: u32 = 0;
static mut icddcr: u32 = 0;
const QEMU_ICDICTR : u32 = 0x00000402; // 96 interrupts, 64 external interrupt lines.

impl ::cpu::core::CPU {
    // p. 1220
    pub fn do_irq(&mut self, irq: u32) -> Option<u32> {
        let irq_masked = self.get_cpsr_bit(::cpu::cpsr::CPSR_I) == 1;
        let gic_enabled = unsafe { iccicr == 1 } && unsafe { icddcr == 1 };
        if !gic_enabled {
            unsafe {
                println!("irq is disabled: {} {} {}", self.get_cpsr_bit(::cpu::cpsr::CPSR_I) == 0, iccicr, icddcr);
            }
            return None;
        }

        unsafe {
            icciar = irq;
            irq_status = match irq_status {
                IrqStatus::Inactive      => IrqStatus::Pending,
                IrqStatus::Pending       => IrqStatus::Pending, // should not happen under normal conditions?
                _                        => {
                    println!("unexpected irq_status: {:#?}", irq_status);
                    return None; // FIXME should not happen but happens
                }
            };
        }

        println!("do_irq(): 0x{:x} ({})", irq, if irq_masked {"masked"} else {"not masked"});
        if irq_masked {
            return None;
        }

        // Store the CPSR to the SPSR of the exception mode.
        self.spsr[::cpu::core::ProcessorMode::Irq as usize] = self.cpsr;
        /* PC is stored in the LR of the exception mode.
           Link register is set to a specific address based on the current instruction. */
        self.regs[::cpu::core::ProcessorMode::Irq as usize][14] = self.get_pc() + 4;
        println!("processing interrupt, lr for irq is set to 0x{:x}", self.get_pc() + 4);
        // Update the CPSR about the exception
        self.set_cpsr_bits(::cpu::cpsr::CPSR_M, ::cpu::core::ProcessorMode::Irq as u32);
        self.set_cpsr_bit(::cpu::cpsr::CPSR_I, 1);

        // Set the PC to the address of the exception handler.
        Some(0xFFFF0018 as u32)
    }

    //  When an interrupt occurs the ARM11 CPU has to read the Interrupt
    //  Acknowledge register
    //  to get the interrupt number, then it can use a jump table to call the ISR
    //  When the CPU has finished servicing an interrupt, it writes the
    //  interrupt number to the End of Interrupt register.
    pub fn gic_cpu_read(addr: usize, va: usize) -> u32 {
        println!("gic_cpu_read: 0x{:x} (0x{:x})", addr, va);
        let gic_enabled = unsafe { iccicr == 1 } && unsafe { icddcr == 1 };
        match addr {
            0xc => {
                if !gic_enabled {
                    println!("returned {:x} due to gic being disabled", SPURIOUS_IRQ);
                    return SPURIOUS_IRQ;
                }
                unsafe {
                    let (new_irq_status, ret) = match irq_status {
                        IrqStatus::Inactive => (IrqStatus::Inactive, SPURIOUS_IRQ),
                        IrqStatus::Pending => (IrqStatus::Active, icciar),
                        _                    => {
                            println!("unexpected irq_status: {:#?}", irq_status);
                            unreachable!()
                        }
                    };
                    irq_status = new_irq_status;
                    println!("returned {:x}", ret);
                    ret
                }
            }
            _ => { println!("unhandled gic_cpu_read: 0x{:x}", addr); unreachable!() },
        }
    }

    // MPCore p. 62
    pub fn gic_cpu_write(addr: usize, va: usize, val: usize) {
        println!("gic_cpu_write: 0x{:x} at 0x{:x} (0x{:x})", val, addr, va);
        match addr {
            0x00 => unsafe { iccicr = val as u32; assert!(val == 0x1) },
            0x04 => assert!(val == 0xF0), // ICCPMR, priority threshold
            0x10 => { // ICCEOIR, active -> inactive
                unsafe {
                    irq_status = match irq_status {
                        IrqStatus::Active => { icciar = SPURIOUS_IRQ; IrqStatus::Inactive},
                        _ => { println!("{:#?}", irq_status); unreachable!() },
                    };
                }
            },
            _ => { println!("unhandled gic_cpu_write: 0x{:x}", addr); unreachable!() },
        }
    }

    // MPCore p. 52
    pub fn gic_dist_read(addr: usize, va: usize) -> u32 {
        println!("gic_dist_read: 0x{:x} (0x{:x})", addr, va);
        match addr {
            0x4 => QEMU_ICDICTR,
            _ => { println!("unhandled gic_dist_read: 0x{:x}", addr); unreachable!() },
        }
    }

    // MPCore p. 52
    pub fn gic_dist_write(addr: usize, _: usize, val: usize) {
        println!("gic_dist_write: 0x{:x} at 0x{:x}", val, addr);
        match addr {
            0x00  => unsafe { icddcr = val as u32; assert!(val == 0 || val == 1) },
            0x100 ... 0x11c => {}, // ICDISERn, enabling
            0x180 ... 0x188 => {}, // ICDICERn, disabling
            0x400 ... 0x45c => {}, // ICDIPRn (priorities), 0xa0a0a0a0 
            0x820 ... 0x85c => {}, // ICDIPTRn (targets), 0x1010101 
            0xc08 ... 0xc14 => {}, // ICDICFRn, edge/level, etc
            _ => { println!("unhandled gic_dist_write: 0x{:x}", addr); unreachable!() },
        }
    }
}
