use std::io::{Write};
use num::FromPrimitive;

pub static mut tx_irq: bool = false;
pub static mut rx_irq: bool = false;
static mut input: [char; 128] = ['\0'; 128];
static mut input_l: usize = 0;
static mut input_r: usize = 0;

static mut tirq_mask: bool = false;
static mut rirq_mask: bool = false;
static mut uart_cr: u32 = 0x300;
static mut uart_crh: u32 = 0x10;

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

pub unsafe fn push_input_char(ch: char) {
    assert!(input_r < 128);
    input[input_r] = ch;
    input_r += 1;
    update_rx_irq_flag();
}

unsafe fn get_mis() -> u32 {
        let mut ret = 0;
        if tx_irq {
            ret |= 1 << 5; // TXMIS
        }
        if rx_irq {
            ret |= 1 << 4; // RXMIS
        }

        ret
}

unsafe fn get_fr() -> u32 {
    if input_l == input_r || input_r == 0 {
        0x90 // TXFE and RXFE set
    } else {
        0x80 // only TXFE set
    }
}

unsafe fn get_dr() -> u32 {
    assert!(input_l < input_r);

    let val = input[input_l] as u32;
    input_l += 1;
    val
}

pub fn uart_read(addr: usize, _: usize) -> u32 {
    let reg = UARTRegs::from_usize(addr);
    let value: u32 = match reg {
        Some(UARTRegs::DR)        => unsafe { rx_irq = false; get_dr() },
        Some(UARTRegs::FR)        => unsafe { get_fr() }, // flag register
        Some(UARTRegs::CR)        => unsafe { uart_cr },
        Some(UARTRegs::MIS)       => unsafe { get_mis() }, // masked interrupt status
        Some(UARTRegs::LCRH)      => unsafe { uart_crh },
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

    value
}

unsafe fn tx_enabled() -> bool {
    ::util::get_bit(uart_cr, 0) == 1 && // UART enabled
    ::util::get_bit(uart_cr, 8) == 1    // TX enabled
}

pub unsafe fn update_tx_irq_flag() {
    tx_irq = tx_enabled() && tirq_mask;
}

unsafe fn rx_enabled() -> bool {
    ::util::get_bit(uart_cr, 0) == 1 && // UART enabled
    ::util::get_bit(uart_cr, 9) == 1    // RX enabled
}

pub unsafe fn update_rx_irq_flag() {
    rx_irq = input_l < input_r && rx_enabled() && rirq_mask;
}

pub fn uart_write(addr: usize, va: usize, value: usize) {
    let reg = UARTRegs::from_usize(addr);
    match reg {
        Some(UARTRegs::DR) => {
            assert!(value < ::std::u8::MAX as usize);
            write!(&mut ::std::io::stderr(), "{}", value as u8 as char).unwrap();
            unsafe { update_tx_irq_flag(); }
        },
        Some(UARTRegs::IBRD) | // integer baud rate
        Some(UARTRegs::FBRD) => {}, // fractional baud rate
        Some(UARTRegs::LCRH) => unsafe { // line control
            uart_crh = value as u32;
        },
        Some(UARTRegs::CR) => unsafe {
            uart_cr = value as u32;
            update_tx_irq_flag();
            update_rx_irq_flag();
        },
        Some(UARTRegs::IFLS) => {}, // interrupt FIFO level select
        Some(UARTRegs::IMSC) => unsafe { // interrupt mask set/clear
            tirq_mask = ::util::get_bit(value as u32, 5) == 1;
            rirq_mask = ::util::get_bit(value as u32, 4) == 1;
            update_tx_irq_flag();
            update_rx_irq_flag();
        },
        Some(UARTRegs::ICR) => unsafe { // interrupt clear register
            if ::util::get_bit(value as u32, 5) == 1 { // TXIC
                tx_irq = false;
            }
            if ::util::get_bit(value as u32, 4) == 1 { // RXIC
                rx_irq = false;
            }
        },
        _ => {
            println!("unhandled uart_write: 0x{:x} at 0x{:x} (0x{:x})", value, addr, va);
            unreachable!();
        },
    }
}
