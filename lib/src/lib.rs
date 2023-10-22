#![crate_type = "lib"]
#![no_std]

//pub const MEMORY_MAP_SIZE: usize = 100;
pub const MEMORY_MAP_SIZE: usize = 8000;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct KernelArguments {
    pub frame_buffer_info:      FrameBufferInfo,
    pub mode_info:              MyModeInfo,
//    pub memory_map:             MemoryMap,
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
pub struct MyModeInfo {
    pub version: u32, // must 0
    pub hor_res: u32,
    pub ver_res: u32,
    pub format: PixelFormat,
    pub mask: Option<PixelBitmask>,
    pub stride: u32,
}

#[cfg(feature = "uefi-feature")]
impl From<uefi::proto::console::gop::ModeInfo> for MyModeInfo {
    fn from(value: uefi::proto::console::gop::ModeInfo) -> Self {
        let pixel_format = match value.pixel_format() {
            uefi::proto::console::gop::PixelFormat::Bgr
                => PixelFormat::Bgr,
            uefi::proto::console::gop::PixelFormat::Bitmask
                => PixelFormat::Bitmask,
            uefi::proto::console::gop::PixelFormat::BltOnly
                => PixelFormat::BltOnly,
            uefi::proto::console::gop::PixelFormat::Rgb
                => PixelFormat::Rgb,
        };

        let pixel_bit_mask = match value.pixel_bitmask() {
            None => None,
            _ => Some(PixelBitmask {
                red:            value.pixel_bitmask().unwrap().red,
                green:          value.pixel_bitmask().unwrap().green,
                blue:           value.pixel_bitmask().unwrap().blue,
                reserved:       value.pixel_bitmask().unwrap().reserved,
            }),
        };

        MyModeInfo {
            version: 0,
            hor_res: value.resolution().0 as u32,
            ver_res: value.resolution().1 as u32,
            format: pixel_format,
            mask: pixel_bit_mask,
            stride: value.stride() as u32,
        }
    }
}

// #[repr(C)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct MemoryMapIter<'buffer> {
//     descriptor: &'buffer [MemoryDescriptor; MEMORY_MAP_SIZE],
//     index: usize,
// }

// impl<'buffer> Iterator for MemoryMapIter<'buffer> {
//     type Item = MemoryDescriptor;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.index += 1;

//         if self.index >= self.descriptor.len() {
//             None
//         } else {
//             Some(self.descriptor[self.index])
//         }
//     }
// }

// #[cfg(feature = "uefi-feature")]
// impl<'buffer> From<uefi::table::boot::MemoryMapIter<'buffer>> for MemoryMapIter<'buffer> {
//     fn from(value: uefi::table::boot::MemoryMapIter<'buffer>) -> Self {
//         let mut _descriptor: &'buffer [MemoryDescriptor; MEMORY_MAP_SIZE] =
//             &[Default::default(); MEMORY_MAP_SIZE];

//         for (i, val) in value.enumerate() {
//             _descriptor[i] = (*val).into();
//         }

//         MemoryMapIter {
//             descriptor: &_descriptor,
//             index: 0,
//         }
//     }
// }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryMap {
    pub map: [MemoryDescriptor; MEMORY_MAP_SIZE],
    pub len: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct MemoryDescriptor {
    pub memory_type: MemoryType,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[cfg(feature = "uefi-feature")]
impl From<uefi::table::boot::MemoryDescriptor> for MemoryDescriptor {
    fn from(value: uefi::table::boot::MemoryDescriptor) -> Self {
        MemoryDescriptor {
            memory_type: value.ty.into(),
            physical_start: value.phys_start,
            virtual_start: value.virt_start,
            number_of_pages: value.page_count,
            attribute: value.att.bits(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum MemoryType {
    /// This enum variant is not used.
    #[default]
    RESERVED = 0,
    /// The code portions of a loaded UEFI application.
    LoaderCode = 1,
    /// The data portions of a loaded UEFI applications,
    /// as well as any memory allocated by it.
    LoaderData = 2,
    /// Code of the boot drivers.
    ///
    /// Can be reused after OS is loaded.
    BootServicesCode = 3,
    /// Memory used to store boot drivers' data.
    ///
    /// Can be reused after OS is loaded.
    BootServicesData = 4,
    /// Runtime drivers' code.
    RuntimeServicesCode = 5,
    /// Runtime services' code.
    RuntimeServicesData = 6,
    /// Free usable memory.
    Conventional = 7,
    /// Memory in which errors have been detected.
    Unstable = 8,
    /// Memory that holds ACPI tables.
    /// Can be reclaimed after they are parsed.
    AcpiReclaim = 9,
    /// Firmware-reserved addresses.
    AcpiNonVolatile = 10,
    /// A region used for memory-mapped I/O.
    Mmio = 11,
    /// Address space used for memory-mapped port I/O.
    MmioPortSpace = 12,
    /// Address space which is part of the processor.
    PalCode = 13,
    /// Memory region which is usable and is also non-volatile.
    PersistentMemory = 14,
}

#[cfg(feature = "uefi-feature")]
impl From<uefi::table::boot::MemoryType> for MemoryType {
    fn from(value: uefi::table::boot::MemoryType) -> Self {
        match value {
            uefi::table::boot::MemoryType::RESERVED => Self::RESERVED,
            uefi::table::boot::MemoryType::LOADER_CODE => Self::LoaderCode,
            uefi::table::boot::MemoryType::LOADER_DATA => Self::LoaderCode,
            uefi::table::boot::MemoryType::BOOT_SERVICES_CODE => Self::BootServicesCode,
            uefi::table::boot::MemoryType::BOOT_SERVICES_DATA => Self::BootServicesData,
            uefi::table::boot::MemoryType::RUNTIME_SERVICES_CODE => Self::RuntimeServicesCode,
            uefi::table::boot::MemoryType::RUNTIME_SERVICES_DATA => Self::RuntimeServicesData,
            uefi::table::boot::MemoryType::CONVENTIONAL => Self::Conventional,
            uefi::table::boot::MemoryType::UNUSABLE => Self::Unstable,
            uefi::table::boot::MemoryType::ACPI_RECLAIM => Self::AcpiReclaim,
            uefi::table::boot::MemoryType::ACPI_NON_VOLATILE => Self::AcpiNonVolatile,
            uefi::table::boot::MemoryType::MMIO => Self::Mmio,
            uefi::table::boot::MemoryType::MMIO_PORT_SPACE => Self::MmioPortSpace,
            uefi::table::boot::MemoryType::PAL_CODE => Self::PalCode,
            uefi::table::boot::MemoryType::PERSISTENT_MEMORY => Self::PersistentMemory,
            _ => panic!(),
        }
    }
}
