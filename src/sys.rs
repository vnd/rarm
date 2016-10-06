use time::precise_time_ns;

pub fn sys_read(addr: usize, va: usize) -> u32 {
    let ret = match addr {
        0x48 => 0, // SYS_MCI
        0x5c => (precise_time_ns() / 1000 * 24) as u32, // SYS_24MHZ
        0x84 => 0xc000191, // SYS_PROCID0, tile id
        0xa8 => 1, // SYS_CFGSTAT
        _ => { println!("unhandled sys_read: 0x{:x} (0x{:x})", addr, va); unreachable!() },
    };

	println!("sys_read: 0x{:x} (0x{:x}) => 0x{:x}", addr, va, ret);
    ret
}
