#![no_std]
#![no_main]
//#![feature(asm)]

mod graphics;
mod font;
mod console;
mod devices;
mod kernel_lib;
mod interrupt;

use core::arch::asm;
use core::fmt::Write;

use lib::{
    KernelArguments,
};

use graphics::*;
use console::*;

#[no_mangle]
pub extern "efiapi" fn kernel_main(
    args: &KernelArguments
) {
    
    match args.mode_info {
        Rgb => {
            let pixel_writer_rgb = 
                RGBResv8BitPerColorPixelWriter::new(
                    args.frame_buffer_info.fb,
                    args.frame_buffer_info.size as u32,
                    args.mode_info.hor_res,
                    args.mode_info.ver_res,
                    args.mode_info.stride,
                );

            let hor_res = args.mode_info.hor_res;
            let ver_res = args.mode_info.ver_res;
            let pixel_color = {
                PixelColor {
                    r:  255,
                    g:  255,
                    b:  255
                }
            };
            
            for x in 0..hor_res {
                for y in 0..ver_res {
                    pixel_writer_rgb.write(x, y, &pixel_color);
                }
            }

            let pixel_color = {
                PixelColor {
                    r:  0,
                    g:  255,
                    b:  0
                }
            };
            
            for x in 0..1000 {
                for y in 0..1000 {
                    pixel_writer_rgb.write(x, y, &pixel_color);
                }
            }

            let mut console_writer = ConsoleWriter::new(&pixel_writer_rgb);
            write!(&mut console_writer, "Welcome to MikanOS\n").unwrap();        
        },
        Bgr => {
            let pixel_writer_bgr = 
                BGRResv8BitPerColorPixelWriter::new(
                    args.frame_buffer_info.fb,
                    args.frame_buffer_info.size as u32,
                    args.mode_info.hor_res,
                    args.mode_info.ver_res,
                    args.mode_info.stride,
                );

            let hor_res = args.mode_info.hor_res;
            let ver_res = args.mode_info.ver_res;
            let pixel_color = {
                PixelColor {
                    r:  255,
                    g:  255,
                    b:  255
                }
            };
                
            for x in 0..hor_res {
                for y in 0..ver_res {
                    pixel_writer_bgr.write(x, y, &pixel_color);
                }
            }

            let pixel_color = {
                PixelColor {
                    r:  0,
                    g:  255,
                    b:  0
                }
            };
            
            for x in 0..200 {
                for y in 0..200 {
                    pixel_writer_bgr.write(x, y, &pixel_color);
                }
            }
            
            let mut console_writer = ConsoleWriter::new(&pixel_writer_bgr);
            write!(&mut console_writer, "Welcome to MikanOS\n").unwrap();        
        },
        _=> {
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