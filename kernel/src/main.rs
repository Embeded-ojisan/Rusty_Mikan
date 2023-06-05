#![no_std]
#![no_main]
//#![feature(asm)]

use core::arch::asm;
use lib::KernelArguments;
use log::info;

#[no_mangle]
pub extern "efiapi" fn kernel_main(
    args: &KernelArguments
) {
    info!("kernel");

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