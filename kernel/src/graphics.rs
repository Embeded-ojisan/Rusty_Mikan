
use lib::{
    PixelFormat,
};

pub trait PixelWriter {
    fn write(
        &self,
        x:          u32,
        y:          u32,
        c:          &PixelColor,
    );

    fn pixel_at(
        &self,
        x:          u32,
        y:          u32,
    ) -> *mut u8;
}

pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
        _size:       u32,
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

pub struct BGRResv8BitPerColorPixelWriter {
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
        c:          &PixelColor,
    ) {
        let p = Self::pixel_at(
            &self,
            x,
            y
        );
    
        let p = 
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

    fn pixel_at(
        &self,
        x:          u32,
        y:          u32,
    ) -> *mut u8 {
        (
            self.config_.frame_buffer as u32
            + 4*(self.config_.pixels_per_scan_line*y +x)
        ) as *mut u8
    }
}

pub struct RGBResv8BitPerColorPixelWriter {
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
        c:          &PixelColor,
    ) {
        let p = Self::pixel_at(
            &self,
            x,
            y
        );
    
        let p = 
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

    fn pixel_at(
        &self,
        x:          u32,
        y:          u32,
    ) -> *mut u8 {
        (
            self.config_.frame_buffer as u32
            + 4*(self.config_.pixels_per_scan_line*y +x)
        ) as *mut u8
    }
}
