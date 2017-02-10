#[macro_use] extern crate enum_primitive;
extern crate byteorder;
extern crate capstone;
extern crate getopts;
extern crate time;
extern crate num;

pub mod memory;
pub mod cpu;
pub mod util;
pub mod arith;
pub mod sys;
pub mod timer;
pub mod uart;

use std::io::{stdin, Read};
use std::env;
use getopts::Options;
use memory::Memory;
use cpu::core::CPU;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn common_read(addr: usize, va: usize) -> u32 {
    println!("common_read: 0x{:x} (0x{:x})", addr, va);
    0
}

fn common_write(addr: usize, va: usize, value: usize) {
    println!("common_write: 0x{:x} at 0x{:x} (0x{:x})", value, addr, va);
}

// FIXME static ref introduced only to pass as a callback
const COMMON_READ_REF: &'static Fn(usize, usize) -> u32 = &common_read;
const COMMON_WRITE_REF: &'static Fn(usize, usize, usize) = &common_write;
const UART_READ_REF: &'static Fn(usize, usize) -> u32 = &::uart::uart_read;
const UART_WRITE_REF: &'static Fn(usize, usize, usize) = &::uart::uart_write;
const SYS_READ_REF: &'static Fn(usize, usize) -> u32 = &sys::sys_read;
const GIC_CPU_READ_REF: &'static Fn(usize, usize) -> u32 = &CPU::gic_cpu_read;
const GIC_CPU_WRITE_REF: &'static Fn(usize, usize, usize) = &CPU::gic_cpu_write;
const GIC_DIST_READ_REF: &'static Fn(usize, usize) -> u32 = &CPU::gic_dist_read;
const GIC_DIST_WRITE_REF: &'static Fn(usize, usize, usize) = &CPU::gic_dist_write;
const SP804_READ_REF: &'static Fn(usize, usize) -> u32 = &timer::sp804_read;
const SP804_WRITE_REF: &'static Fn(usize, usize, usize) = &timer::sp804_write;

// p. 44 in Versatile Express Motherboard Technical Reference Manual
const SYS_BASE:     usize = 0x10000000;
const SP810_BASE:   usize = 0x10001000; // system controller, not publicly documented
const AACI_BASE:    usize = 0x10004000; // audio
const MMCI_BASE:    usize = 0x10005000;
const KMI0_BASE:    usize = 0x10006000;
const KMI1_BASE:    usize = 0x10007000;
const UART0_BASE:   usize = 0x10009000;
const UART1_BASE:   usize = 0x1000a000;
const UART2_BASE:   usize = 0x1000b000;
const UART3_BASE:   usize = 0x1000c000;
const WDT_BASE:     usize = 0x1000f000;
const SP804_BASE:   usize = 0x10011000; // dual timer
const RTC_BASE:     usize = 0x10017000;
const CLCDC_BASE:   usize = 0x10020000;
const PL341_BASE:   usize = 0x100e0000;
const PL354_BASE:   usize = 0x100e1000;
const GPIO_BASE:    usize = 0x100e8000;
const GIC_CPU_BASE: usize = 0x1e000100;
const GIC_DIST_BASE:usize = 0x1e001000;
const L2CC_BASE:    usize = 0x1e00a000;
const SMSC_BASE:    usize = 0x4e000000;
const USB_BASE:     usize = 0x4f000000;

const RAM_BASE:     usize = 0x60000000;
const RAM_SIZE:     usize = 16*1024*1024;

const ATAGS_ADDR:   usize = 0x60000100;
const ZIMAGE_ADDR:  usize = 0x60010000;
const RAMDISK_ADDR: usize = 0x60800000;

struct CmdlineArgs {
    bp: u32,
    bpc: u64,
    ignore_bp: u32,
    wp: u32,
    verbose: bool,
}

fn parse_cmdline_args() -> CmdlineArgs {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("b", "break", "set a breakpoint", "hex");
    opts.optopt("i", "ignore", "ignore the given breakpoint specified number of times", "dec");
    opts.optopt("n", "count break", "break after n instruction are executed", "dec");
    opts.optopt("w", "watchpoint", "set a watchpoint", "hex");
    opts.optflag("v", "verbose", "more verbose output");
    let matches = opts.parse(&args[1..]).unwrap();
    let bp: u32 = if matches.opt_present("b") {
        u32::from_str_radix(&matches.opt_str("b").unwrap().as_str()[2..], 16).unwrap()
    } else { 0 };
    let bpc: u64 = if matches.opt_present("n") {
        u64::from_str_radix(matches.opt_str("n").unwrap().as_str(), 10).unwrap()
    } else { 0 };
    let ignore_bp: u32 = if matches.opt_present("i") {
        u32::from_str_radix(matches.opt_str("i").unwrap().as_str(), 10).unwrap()
    } else { 0 };
    let wp: u32 = if matches.opt_present("w") {
        u32::from_str_radix(&matches.opt_str("w").unwrap().as_str()[2..], 16).unwrap()
    } else { 0 };

    let verbose = matches.opt_present("v");

    CmdlineArgs {
        bp: bp,
        bpc: bpc,
        ignore_bp: ignore_bp,
        wp: wp,
        verbose: verbose,
    }
}

fn add_memory_regions(mem: &mut Memory) {
    mem.add_region("RAM",      RAM_BASE,      RAM_SIZE,   None,                    None);
    mem.add_region("UART0",    UART0_BASE,    0x1000,     Some(UART_READ_REF),     Some(UART_WRITE_REF));
    mem.add_region("SYS",      SYS_BASE,      0x1000,     Some(SYS_READ_REF),      Some(COMMON_WRITE_REF));
    mem.add_region("GIC_CPU",  GIC_CPU_BASE,  0x100,      Some(GIC_CPU_READ_REF),  Some(GIC_CPU_WRITE_REF));
    mem.add_region("GIC_DIST", GIC_DIST_BASE, 0x1000,     Some(GIC_DIST_READ_REF), Some(GIC_DIST_WRITE_REF));
    mem.add_region("SP804",    SP804_BASE,    0x1000,     Some(SP804_READ_REF),    Some(SP804_WRITE_REF));
    mem.add_region("SP810",    SP810_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("MMCI",     MMCI_BASE,     0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("KMI0",     KMI0_BASE,     0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("KMI1",     KMI1_BASE,     0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("WDT",      WDT_BASE,      0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("RTC",      RTC_BASE,      0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("CLCDC",    CLCDC_BASE,    0x10000,    None,                    Some(COMMON_WRITE_REF));
    mem.add_region("PL341",    PL341_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("PL354",    PL354_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("GPIO",     GPIO_BASE,     0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("UART1",    UART1_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("UART2",    UART2_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("UART3",    UART3_BASE,    0x1000,     None,                    Some(COMMON_WRITE_REF));
    mem.add_region("SMSC",     SMSC_BASE,     0x1000000,  Some(COMMON_READ_REF),   Some(COMMON_WRITE_REF));
    mem.add_region("USB",      USB_BASE,      0x1000000,  Some(COMMON_READ_REF),   Some(COMMON_WRITE_REF));
    mem.add_region("L2CC",     L2CC_BASE,     0x1000,     Some(COMMON_READ_REF),   Some(COMMON_WRITE_REF));
    mem.add_region("AACI",     AACI_BASE,     0x1000,     Some(COMMON_READ_REF),   Some(COMMON_WRITE_REF));
    mem.add_region("FAULTS",   0,             0x1000,     None,                    None); // basically a workaround
}

fn main() {
    let mut mem: Memory = Memory::new();
    add_memory_regions(&mut mem);
    mem.load(ATAGS_ADDR,   "bin/atags");
    mem.load(ZIMAGE_ADDR,  "bin/zImage");
    mem.load(RAMDISK_ADDR, "bin/initramfs");

    let args = parse_cmdline_args();
    if args.wp > 0 {
        mem.add_watchpoint(args.wp);
    }

    let mut cpu: CPU = CPU::new(mem, ZIMAGE_ADDR as u32);
    // r0 = 0,
    cpu.set_reg(0, 0);
    // r1 = machine type number (Versatile Express)
    cpu.set_reg(1, 2272);
    /*  r2 = physical address of tagged list in system RAM, or
             physical address of device tree block (dtb) in system RAM */
    cpu.set_reg(2, ATAGS_ADDR as u32);

    if args.bp > 0 {
        cpu.set_breakpoint(args.bp, args.ignore_bp);
    }
    if args.bpc > 0 {
        cpu.set_count_breakpoint(args.bpc);
    }

    let capstone =
        capstone::HandleBuilder::new(capstone::ffi::CsArch::ARCH_ARM,
                                     capstone::ffi::mode::ARM).detail().build().unwrap();

    let (tx, rx): (Sender<char>, Receiver<char>) = mpsc::channel();

    thread::spawn(move || {
        let mut character = [0];
        loop {
            if let Ok(_) = stdin().read(&mut character) {
                tx.send(character[0] as char).unwrap();
            }
        }
    });

    thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Ok(ch) => unsafe {
                    ::uart::push_input_char(ch);
                },
                Err(_) => {}
            }
        }
    });

    unsafe { cpu.start(capstone, args.verbose); }
}
