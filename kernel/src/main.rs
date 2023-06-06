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
    unsafe {
        let buffer = core::slice::from_raw_parts_mut(
            (args.frame_buffer_info).fb,
            (args.frame_buffer_info).size,
        );
        let mut i = 0;
        loop {
//                info!("frame_buffer_info size is {}", i);
            if i > (args.frame_buffer_info).size {
                break;
            }
            buffer[i] = (i%256) as u8;
            i = i+1;
        }
    }
    
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