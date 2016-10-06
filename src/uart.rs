use std::io::{Write};
use num::FromPrimitive;

pub static mut tx_irq: bool = false;
static mut fifo_enabled: bool = false;
static mut tirq_mask: bool = false;
static mut uart_cr: u32 = 0x300;

enum_from_primitive! {
#[derive(Debug, Clone)]
enum UARTRegs {
    DR        = 0x00,
    FR        = 0x18,
    IBRD      = 0x24,
    FBRD      = 0x28,
    LCRH      = 0x2c,
    CR        = 0x30,
    IFLS      = 0x34,
    IMSC      = 0x38,
    MIS       = 0x40,
    ICR       = 0x44,
    PERIPHID0 = 0xfe0,
    PERIPHID1 = 0xfe4,
    PERIPHID2 = 0xfe8,
    PERIPHID3 = 0xfec,
    PCELLID0  = 0xff0,
    PCELLID1  = 0xff4,
    PCELLID2  = 0xff8,
    PCELLID3  = 0xffc,
}
}

pub fn uart_read(addr: usize, _: usize) -> u32 {
    let reg = UARTRegs::from_usize(addr);
    let ret: u32 = match reg {
        Some(UARTRegs::FR)        => { 0x90 },
        Some(UARTRegs::CR)        => unsafe { uart_cr },
        Some(UARTRegs::MIS)       => if unsafe { tirq_mask } { 1 << 5 } else { 0 },
        Some(UARTRegs::PERIPHID0) => { 0x11 }, 
        Some(UARTRegs::PERIPHID1) => { 0x10 }, 
        Some(UARTRegs::PERIPHID2) => { 0x14 }, 
        Some(UARTRegs::PERIPHID3) => { 0x00 }, 
        Some(UARTRegs::PCELLID0)  => { 0x0d }, 
        Some(UARTRegs::PCELLID1)  => { 0xf0 }, 
        Some(UARTRegs::PCELLID2)  => { 0x05 }, 
        Some(UARTRegs::PCELLID3)  => { 0xb1 }, 
        _ => { println!("unhandled uart_read: 0x{:x}", addr); unreachable!() },
    };
    ret
}

pub unsafe fn update_tx_irq_flag() {
    let tx_enabled = ::util::get_bit(uart_cr, 0) == 1 &&
                     ::util::get_bit(uart_cr, 8) == 1;

    tx_irq = tx_enabled && tirq_mask;
    println!("tx_irq is now {}", tx_irq);
}

pub fn uart_write(addr: usize, va: usize, value: usize) {
    let reg = UARTRegs::from_usize(addr);
    match reg {
		Some(UARTRegs::DR) => {
			assert!(value < ::std::u8::MAX as usize);
			write!(&mut ::std::io::stderr(), "{}", value as u8 as char).unwrap();
		},
        Some(UARTRegs::IBRD) => {},
        Some(UARTRegs::FBRD) => {},
        Some(UARTRegs::LCRH) => unsafe {
            fifo_enabled = ::util::get_bit(value as u32, 4) == 1;
        }, 
        Some(UARTRegs::CR) => unsafe {
            uart_cr = value as u32;
            update_tx_irq_flag();
        }, 
        Some(UARTRegs::IFLS) => {},
        Some(UARTRegs::IMSC) => unsafe {
            tirq_mask = ::util::get_bit(value as u32, 5) == 1;
            update_tx_irq_flag();
        },
        Some(UARTRegs::ICR) => unsafe {
            if ::util::get_bit(value as u32, 5) == 1 {
                tx_irq = false;
            }
        },
		_ => {
			println!("unhandled uart_write: 0x{:x} at 0x{:x} (0x{:x})", value, addr, va);
			unreachable!();
		},
    }

    if addr > 0 {
        println!("uart_write: {:?} => 0x{:x} at 0x{:x}", reg, value, va);
    }
}
