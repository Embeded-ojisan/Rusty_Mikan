#![crate_type = "lib"]
#![no_std]

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KernelArguments {
//    pub frame_buffer_info: FrameBufferInfo,
//    pub mode_info: ModeInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FrameBufferInfo {
    pub fb: *mut u8,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb = 0,
    Bgr,
    Bitmask,
    BltOnly,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PixelBitmask {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModeInfo {
    pub version: u32, // must 0
    pub hor_res: u32,
    pub ver_res: u32,
    pub format: PixelFormat,
    pub mask: Option<PixelBitmask>,
    pub stride: u32,
}

#[cfg(feature = "uefi-feature")]
impl From<uefi::proto::console::gop::ModeInfo> for ModeInfo {
    fn from(value: uefi::proto::console::gop::ModeInfo) -> Self {
        let pixel_format = match value.pixel_format() {
            uefi::proto::console::gop::PixelFormat::Bgr => PixelFormat::Bgr,
            uefi::proto::console::gop::PixelFormat::Bitmask => PixelFormat::Bitmask,
            uefi::proto::console::gop::PixelFormat::BltOnly => PixelFormat::BltOnly,
            uefi::proto::console::gop::PixelFormat::Rgb => PixelFormat::Rgb,
        };

        let pixel_bit_mask = match value.pixel_bitmask() {
            None => None,
            _ => Some(PixelBitmask {
                red: value.pixel_bitmask().unwrap().red,
                green: value.pixel_bitmask().unwrap().green,
                blue: value.pixel_bitmask().unwrap().blue,
                reserved: value.pixel_bitmask().unwrap().reserved,
            }),
        };

        ModeInfo {
            version: 0,
            hor_res: value.resolution().0 as u32,
            ver_res: value.resolution().1 as u32,
            format: pixel_format,
            mask: pixel_bit_mask,
            stride: value.stride() as u32,
        }
    }
}

