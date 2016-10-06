use std::ops::Range;
use util::{get_bit, get_bits};

const TIMER_EN:         u32 = 7;
const TIMER_MODE:       u32 = 6;
const TIMER_INT:        u32 = 5;
const TIMER_PRE: Range<u32> = 2..3;
const TIMER_SIZE:       u32 = 1;
const TIMER_ONE_SHOT:   u32 = 0;

struct Timer {
    value: u32,
    load: u32,
    ctrl: u32,
}

static mut timer1: Timer = Timer {value: 0xFFFFFFFF, load: 0, ctrl: 0b00100000};
static mut timer2: Timer = Timer {value: 0xFFFFFFFF, load: 0, ctrl: 0b00100000};

//TODO rewrite in idiomatic Rust

fn timer_enabled(timer: *const u32) -> bool {
    unsafe { get_bit(*timer, TIMER_EN) == 1 }
}

fn timer_ctrl(timer: *mut u32, val: u32) {
    // Timer1Control, 0x0, 0x22 (free running, int enable), 0xe2 (periodic, timer enable, interrupt enable)
    // Timer2Control, 0x0, 0xc2 (timer enable, periodic, interrupt disable)
    assert!(get_bit(val, TIMER_ONE_SHOT) == 0);
    assert!(!(get_bit(val, TIMER_EN) == 1 && get_bit(val, TIMER_SIZE) == 0)); // support only 32 bit counters
    assert!(get_bits(val, TIMER_PRE) == 0);
    unsafe { *timer = val; }
}

fn timer_set_load(load: *mut u32, value: *mut u32, val: u32) {
    unsafe { *load = val; *value = val; }
}

pub fn sp804_read(addr: usize, va: usize) -> u32 {
    unsafe {
        let ret = match addr {
            0x04 => timer1.value,
            0x24 => timer2.value,
            _ => unreachable!(),
        };
        println!("sp804_read: 0x{:x} (0x{:x}) returned 0x{:x}", addr, va, ret);
        return ret;
    }
}

pub fn sp804_write(addr: usize, va: usize, val: usize) {
    println!("sp804_write: 0x{:x} at 0x{:x} (0x{:x})", val, addr, va);
    unsafe {
        match addr {
            0x00 => timer_set_load(&mut timer1.load, &mut timer1.value, val as u32),
            0x08 => timer_ctrl(&mut timer1.ctrl, val as u32),
            0x20 => timer_set_load(&mut timer2.load, &mut timer2.value, val as u32),
            0x24 => {}, // Timer2Value, supposed to be read-only
            0x28 => timer_ctrl(&mut timer2.ctrl, val as u32),
            0x0c => { println!("sp804 Timer1IntClr")},
            0x2c => { println!("sp804 Timer2IntClr")},
            _ => { println!("unhandled sp804_write: 0x{:x}", addr); unreachable!() },
        };
    }
}

unsafe fn _timer_tick(ctrl: *mut u32, value: *mut u32, load: *mut u32, verbose: bool) -> bool {
    let mut ret = false;
    if !timer_enabled(ctrl) {
        return false;
    }

    let step = 1;
    *value = *value - step;
    if *value < step {
        match get_bit(*ctrl, TIMER_MODE) {
            0 => *value = 0xFFFFFFFF, // free running
            1 => *value = *load, // periodic
            _ => unreachable!(),
        }
        ret = get_bit(*ctrl, TIMER_INT) == 1;
        if !ret {
            println!("timer has wrapped but interrupt is disabled (0x{:x})", *ctrl);
        } else {
            if verbose {
                println!("timer has wrapped AND interrupt is enabled (0x{:x})", *ctrl);
            }
        }
    }
    ret
}

pub fn timer_tick(verbose: bool) -> (bool, bool) {
    unsafe {
        let timer1_int = _timer_tick(&mut timer1.ctrl, &mut timer1.value, &mut timer1.load, verbose);
        let timer2_int = _timer_tick(&mut timer2.ctrl, &mut timer2.value, &mut timer2.load, verbose);
        return (timer1_int, timer2_int)
    }
}

pub fn dump_timers() {
    unsafe {
        println!("TIMER1 control/value/load: 0x{:x} 0x{:x} 0x{:x}", timer1.ctrl, timer1.value, timer1.load);
        println!("TIMER2 control/value/load: 0x{:x} 0x{:x} 0x{:x}", timer2.ctrl, timer2.value, timer2.load);
    }
}
