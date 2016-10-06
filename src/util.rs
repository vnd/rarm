extern crate byteorder;
extern crate num;

use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use self::num::Num;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::ops::Range;
use std::process::Command;
use std::str;
use std::mem::transmute;
use capstone::ffi::detail::{ARMReg, ARMOpData, ARMOp};
use capstone::Insn;

pub fn u16_to_bytes(val: u16, bytes: &mut[u8]) {
    Cursor::new(bytes).write_u16::<LittleEndian>(val as u16).unwrap();
}

pub fn u32_to_bytes(val: u32, bytes: &mut[u8]) {
    Cursor::new(bytes).write_u32::<LittleEndian>(val as u32).unwrap();
}

pub fn bytes_to_u16(bytes: &[u8]) -> u16 {
    Cursor::new(bytes).read_u16::<LittleEndian>().unwrap()
}

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    Cursor::new(bytes).read_u32::<LittleEndian>().unwrap()
}

pub unsafe fn print_insn_mnemonic(insn: &Insn, addr2line: bool) {
    assert!(addr2line == false); // vmlinux is not included in the repo
    if addr2line {
        let addr = format!("{:x}", insn.address());
        let cmd = "addr2line -e ~/balau82/linux-3.2/vmlinux 0x".to_string() + addr.as_str();

        Command::new("bash").arg("-c").arg(cmd).status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
    }
    println!("0x{:x} {} {}", insn.address(), insn.mnemonic().unwrap(), insn.op_str().unwrap());
}

pub fn objdump_insn(bytes: &[u8]) {
    let tmp_path = "/tmp/rarm.tmp";
    let mut f = File::create(tmp_path).unwrap();
    f.write_all(&bytes).unwrap();

    println!("");
    let cmd = "arm-linux-gnu-objdump -EL -b binary -D -marm -d /dev/stdin < ".to_string() +
        tmp_path;
    Command::new("bash").arg("-c").arg(cmd).status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("");
}

pub unsafe fn dump_insn(insn: &Insn, verbose: bool) {
    print_insn_mnemonic(insn, verbose);
    objdump_insn(&insn.bytes[..4]);

    if verbose {
        println!("{:#?}", insn);
        println!("{:#?}", insn.detail().unwrap().data_arm());
    }
}

pub fn set_bits(val: &mut u32, _range: Range<u32>, bits: u32) {
    let mut range = _range;
    let start = range.start;
    range.end += 1;
    for i in range {
        set_bit(val, i, get_bit(bits, i - start));
    }
}

pub fn set_bit(val: &mut u32, bit_num: u32, bit: u32) {
    assert!(bit == 0 || bit == 1);
    assert!(bit_num < 32);

    if bit == 1 {
        *val |= 1 << bit_num;
    }
    else {
        *val &= !(1 << bit_num);
    }
}

pub fn get_bits(val: u32, _range: Range<u32>) -> u32 {
    {
        let mut range = _range;
        let mut mask: u32 = 0;
        let start = range.start;
        range.end += 1;
        for j in range {
            mask = mask | (1 << j);
        }

        (val & mask) >> start
    }
}

pub fn get_bit(val: u32, bit_num: u32) -> u32 {
    assert!(bit_num < 32);
    let ret = get_bits(val, (bit_num..bit_num));
    assert!(ret == 0 || ret == 1);
    ret
}

pub unsafe fn reg_num(data: ARMOpData) -> u8 {
    match data {
        ARMOpData::Reg(r) => cs_reg_num(r),
        ARMOpData::Mem(m) => cs_reg_num(transmute(m.base)),
        _ => { unreachable!() },
    }
}

pub fn cs_reg_num(cs_reg: ARMReg) -> u8 {
    match cs_reg {
        ARMReg::ARM_REG_R0  => 0,
        ARMReg::ARM_REG_R1  => 1,
        ARMReg::ARM_REG_R2  => 2,
        ARMReg::ARM_REG_R3  => 3,
        ARMReg::ARM_REG_R4  => 4,
        ARMReg::ARM_REG_R5  => 5,
        ARMReg::ARM_REG_R6  => 6,
        ARMReg::ARM_REG_R7  => 7,
        ARMReg::ARM_REG_R8  => 8,
        ARMReg::ARM_REG_R9  => 9,
        ARMReg::ARM_REG_R10 => 10,
        ARMReg::ARM_REG_R11 => 11,
        ARMReg::ARM_REG_R12 => 12,
        ARMReg::ARM_REG_SP  => 13,
        ARMReg::ARM_REG_LR  => 14,
        ARMReg::ARM_REG_PC  => 15,
        _ => unreachable!(),
    }
}

pub unsafe fn imm_to_u32(data: ARMOpData) -> u32 {
    match data {
        ARMOpData::Imm(i) => i,
        _ => { unreachable!() },
    }
}

pub fn assert_shift(ops: &[ARMOp]) {
    for op in ops {
        assert!(op.shift_type == 0);
        assert!(op.shift_value == 0);
        assert!(op.subtracted == false);
    }
}

pub fn check_subtracted(ops: &[ARMOp], insn: &Insn) {
    for op in ops {
        if op.subtracted == true && insn.op_str().unwrap().contains("-") {
            println!("subtracted is true!");
            unsafe { print_insn_mnemonic(insn, false); }
            println!("{:#?}", op);
            println!("{:#?}", op.data());
            println!("");
        }
    }
}

pub fn is_zero<T: Num>(val: T) -> u32 {
    if val == T::zero() {
        1
    } else {
        0
    }
}
