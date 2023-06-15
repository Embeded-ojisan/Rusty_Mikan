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
        &self,
        x:          u32,
        y:          u32,
        c:          &PixcelColor,
    );

    fn PixelAt(
        config:     &FrameBufferConfig,
        x:          u32,
        y:          u32,
    ) -> *mut u8 {
        (
            config.frame_buffer as u32
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
    pixels_per_scan_line:       u32,
    horizontal_resolution:      u32,
    vertical_resolution:        u32,
    pixel_format:               PixelFormat,
}

impl FrameBufferConfig {
    fn new(
        buf:        *mut u8,
        size:       u32,
        hor_res:    u32,
        ver_res:    u32,
        ppsl:       u32,

    ) -> Self {
        FrameBufferConfig {
            frame_buffer:           buf,
            pixels_per_scan_line:   ppsl,
            horizontal_resolution:  hor_res,
            vertical_resolution:    ver_res,
            pixel_format:           PixelFormat::Rgb,
        }        
    }
}

struct BGRResv8BitPerColorPixelWriter {
    config_: FrameBufferConfig,
}

impl BGRResv8BitPerColorPixelWriter {
    pub fn new(
        buf:        *mut u8,
        size:       u32,
        hor_res:    u32,
        ver_res:    u32,
        ppsl:       u32,
    ) -> Self {
        Self {
            config_: FrameBufferConfig::new(
                buf,
                size,
                hor_res,
                ver_res,
                ppsl
            )
        }
    }
}

impl PixelWriter for BGRResv8BitPerColorPixelWriter {
    fn write(
        &self,
        x:          u32,
        y:          u32,
        c:          &PixcelColor,
    ) {
        let mut p = Self::PixelAt(
            &self.config_,
            x,
            y
        );
    
        let mut p = 
            unsafe {
                core::slice::from_raw_parts_mut(
                    p,
                    3
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
    pub fn new(
        buf:        *mut u8,
        size:       u32,
        hor_res:    u32,
        ver_res:    u32,
        ppsl:       u32,
    ) -> Self {
        Self {
            config_: FrameBufferConfig::new(
                buf,
                size,
                hor_res,
                ver_res,
                ppsl,
            )
        }
    }
}

impl PixelWriter for RGBResv8BitPerColorPixelWriter {
    fn write(
        &self,
        x:          u32,
        y:          u32,
        c:          &PixcelColor,
    ) {
        let mut p = Self::PixelAt(
            &self.config_,
            x,
            y
        );
    
        let mut p = 
            unsafe {
                core::slice::from_raw_parts_mut(
                    p,
                    3
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