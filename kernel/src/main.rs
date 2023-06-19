#![no_std]
#![no_main]
//#![feature(asm)]

mod graphics;

use core::arch::asm;
use lib::{
    KernelArguments,
    PixelFormat,
};
use log::info;

use graphics::*;

#[no_mangle]
pub extern "efiapi" fn kernel_main(
    args: &KernelArguments
) {
    
    match args.mode_info {
        Rgb => {
            let mut pixel_writer_rgb = 
                RGBResv8BitPerColorPixelWriter::new(
                    args.frame_buffer_info.fb,
                    args.frame_buffer_info.size as u32,
                    args.mode_info.hor_res,
                    args.mode_info.ver_res,
                    args.mode_info.stride,
                );

            let hor_res = args.mode_info.hor_res;
            let ver_res = args.mode_info.ver_res;
            let pixcelColor = {
                PixcelColor {
                    r:  255,
                    g:  255,
                    b:  255
                }
            };
            
            for x in 0..hor_res {
                for y in 0..ver_res {
                    pixel_writer_rgb.write(x, y, &pixcelColor);
                }
            }

            let pixcelColor = {
                PixcelColor {
                    r:  0,
                    g:  255,
                    b:  0
                }
            };
            
            for x in 0..200 {
                for y in 0..100 {
                    pixel_writer_rgb.write(x, y, &pixcelColor);
                }
            }
        },
        Bgr => {
            let mut pixel_writer_bgr = 
                BGRResv8BitPerColorPixelWriter::new(
                    args.frame_buffer_info.fb,
                    args.frame_buffer_info.size as u32,
                    args.mode_info.hor_res,
                    args.mode_info.ver_res,
                    args.mode_info.stride,
                );

            let hor_res = args.mode_info.hor_res;
            let ver_res = args.mode_info.ver_res;
            let pixcelColor = {
                PixcelColor {
                    r:  255,
                    g:  255,
                    b:  255
                }
            };
                
            for x in 0..hor_res {
                for y in 0..ver_res {
                    pixel_writer_bgr.write(x, y, &pixcelColor);
                }
            }

            let pixcelColor = {
                PixcelColor {
                    r:  0,
                    g:  255,
                    b:  0
                }
            };
            
            for x in 0..200 {
                for y in 0..100 {
                    pixel_writer_bgr.write(x, y, &pixcelColor);
                }
            }
        },
        _=> {
            ;
        },
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