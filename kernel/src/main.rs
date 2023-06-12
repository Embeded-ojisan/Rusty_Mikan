#![no_std]
#![no_main]
//#![feature(asm)]

use core::arch::asm;
use lib::{
    KernelArguments,
    PixelFormat,
};
use log::info;

trait PixelWriter {
    fn write(
        config:    &FrameBufferConfig,
        x:      usize,
        y:      usize,
        c:      &PixcelColor,
    );

    fn PixelAt(
        config:    &FrameBufferConfig,
        x:      usize,
        y:      usize,
    ) -> *mut u8 {
        (
            (config.frame_buffer as usize)
            + 4*(config.pixels_per_scan_line*y +x)
        ) as *mut u8
    }
}

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

struct BGRResv8BitPerColorPixelWriter {
    config_: FrameBufferConfig,
}

impl BGRResv8BitPerColorPixelWriter {
    pub fn new(buf: *mut u8, size: u32) -> Self {
        Self {
            config_: FrameBufferConfig::new(buf, size)
        }
    }
}

impl PixelWriter for BGRResv8BitPerColorPixelWriter {
    fn write(
        config:    &FrameBufferConfig,
        x:      usize,
        y:      usize,
        c:      &PixcelColor,
    ) {
        let mut p = Self::PixelAt(
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
}

struct RGBResv8BitPerColorPixelWriter {
    config_: FrameBufferConfig,
}

impl RGBResv8BitPerColorPixelWriter {
    pub fn new(buf: *mut u8, size: u32) -> Self {
        Self {
            config_: FrameBufferConfig::new(buf, size)
        }
    }
}

impl PixelWriter for RGBResv8BitPerColorPixelWriter {
    fn write(
        config:    &FrameBufferConfig,
        x:      usize,
        y:      usize,
        c:      &PixcelColor,
    ) {
        let mut p = Self::PixelAt(
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
    
        p[0] = c.r;
        p[1] = c.g;
        p[2] = c.b;
    }
}

#[no_mangle]
pub extern "efiapi" fn kernel_main(
    args: &KernelArguments
) {
    match args.mode_info {
        Rgb => {
            ;
        },
        Bgr => {
            ;
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