#![feature(asm)]
#![feature(llvm_asm)]
#![feature(abi_efiapi)]
#![no_std]
#![no_main]

mod core_requirements;

use serial::SerialPort;
use core::panic::PanicInfo;
#[macro_use] use efi::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    efi_print!("{}", info);
    loop {}
}

#[no_mangle]
extern fn efi_main(image: EfiHandle, sys_t: *mut EfiSystemTable) -> EfiStatus {
    // TODO: MAKE IT NOT PAGE FAULT >:(a

    let st = unsafe { &mut *sys_t };

    unsafe { register_system_table(sys_t); }

    efi::get_memory_map();

    //unsafe { ((*(*sys_t).boot_services).exit_boot_services)(image, 0); }

    EfiStatus::EfiSuccess
}
