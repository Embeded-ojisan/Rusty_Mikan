#![no_std]
#![no_main]
//#![feature(asm)]

use core::arch::asm;
use lib::KernelArguments;

#[no_mangle]
pub extern "efiapi" fn kernel_main(
    args: &KernelArguments
) {

    loop {
        unsafe { asm!("hlt") }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}