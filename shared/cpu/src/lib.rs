#![no_std]
#![feature(llvm_asm)]

pub unsafe fn out8(port: u16, val: u8)
{
    llvm_asm!("out dx, al" :: "{al}"(val), "{dx}"(port) :: "intel", "volatile");
}

/// Input a byte from `port`
pub unsafe fn in8(port: u16) -> u8
{
    let ret: u8;
    llvm_asm!("in al, dx" : "={al}"(ret) : "{dx}"(port) :: "intel", "volatile");
    ret
}