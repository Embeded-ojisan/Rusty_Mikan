#![no_std]
#![no_main]
//#![feature(asm)]

use core::arch::asm;
use lib::{
    KernelArguments,
    PixelFormat,
};
use log::info;

pub struct PixcelColor {
    r: u8,
    g: u8,
    b: u8,
}

pub struct FrameBufferConfig {
    frame_buffer:               *mut u8,
    pixels_per_scan_line:       usize,
    horizontal_resolution:      usize,
    vertical_resolution:        usize,
    pixel_format:               PixelFormat,
}

impl FrameBufferConfig {
    fn new(buf: *mut u8, size: u32) -> Self {
        FrameBufferConfig {
            frame_buffer:           buf,
            pixels_per_scan_line:   0,
            horizontal_resolution:  0,
            vertical_resolution:    0,
            pixel_format:           PixelFormat::Rgb,
        }        
    }
}

pub struct PixelWriter {
    config_: FrameBufferConfig,
}

impl PixelWriter {
    pub fn new(buf: *mut u8, size: u32) -> Self {
        Self {
            config_: FrameBufferConfig::new(buf, size)
        }
    }
}

pub fn PixelAt(
    config:    &FrameBufferConfig,
    x:      usize,
    y:      usize,
) -> *mut u8 {
    (
        (config.frame_buffer as usize)
        + 4*(config.pixels_per_scan_line*y +x)
    ) as *mut u8
}

pub fn write(
    config:    &FrameBufferConfig,
    x:      usize,
    y:      usize,
    c:      &PixcelColor,
) {
    let mut p = PixelAt(
        config,
        x,
        y
    );

    let mut p = 
        unsafe {
            core::slice::from_raw_parts_mut(
                p,
                3 as usize
            )
        };

    p[0] = c.b;
    p[1] = c.g;
    p[2] = c.r;
}

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