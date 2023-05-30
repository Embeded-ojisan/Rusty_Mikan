#![no_main]
#![no_std]

#![feature(abi_efiapi)]

extern crate alloc;

use uefi::prelude::*;
//use uefi::allocator::*;

use uefi::proto::media::{
    fs::SimpleFileSystem,
    file::*
};

use uefi::proto::loaded_image::LoadedImage;

use uefi::proto::device_path::{
    text::DevicePathFromText,
    build::DevicePathBuilder,
};

use uefi::proto::console::gop::{
//    ModeInfo,
    GraphicsOutput,
};

use uefi::table::boot::*;

use uefi::Identify;

use uefi::CStr16;


use alloc::vec;
use alloc::vec::Vec;
use alloc::string::*;
use log::info;
use alloc::rc::Rc;

use core::option::Option;
use core::ops::DerefMut;
use core::any::type_name;
use core::mem::transmute;
use core::slice::from_raw_parts_mut;

use lib::{KernelArguments};
struct MemmoryMap {
    buffer_size: usize,
    buffer: Option<Vec<u8>>,
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: usize,
}

impl MemmoryMap {
    pub fn new(
        inBuffer_size: usize,
    ) 
    -> Self {
        let buffer = vec![0u8; inBuffer_size];
        MemmoryMap {
            buffer_size:            inBuffer_size,
            buffer:                 Some(buffer),
            map_size:               0,
            map_key:                0,
            descriptor_size:        0,
            descriptor_version:     0,
        }
    }

    pub fn GetMemoryMap(
        &mut self,
        boot_services: &BootServices,
    ) -> Status {
        match self.buffer.as_mut() {
            Some(buf) => {
                boot_services
                    .memory_map(
                        buf.as_mut_slice()
                    );
                Status::SUCCESS
            },
            None => Status::BUFFER_TOO_SMALL,
        }
    }
    pub fn SaveMemoryMap(
        &self, 
        handle: FileHandle
    ) -> Status {
            Status::SUCCESS
    }
}

struct EfiProtocols<'a> {
    mSimpleFileSystem:      ScopedProtocol<'a, SimpleFileSystem>,
//    mDevicePathFromText:      ScopedProtocol<'a, DevicePathFromText>,
}

impl<'a> EfiProtocols<'a> {
    pub fn new(
        image_handle:   &'a Handle,
        boot_services:  &'a uefi::prelude::BootServices,
    ) -> Self {

        EfiProtocols {

            mSimpleFileSystem: 
                boot_services
                    .get_image_file_system(*image_handle)
                    .unwrap(),

/*
            mDevicePathFromText:
                boot_services
                    .open_protocol::<DevicePathFromText>(
                        OpenProtocolParams {
                            boot_services.image_handle(),
                            agent: boot_services.image_handle(),
                            controller: None,
                        },
                        OpenProtocolAttributes::GetProtocol
                ).unwrap(),
*/
        }
    }
}

fn type_of<T>(_: T) -> String{
    let a = core::any::type_name::<T>();
    return a.to_string();
  }

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");

    // 前処理
    let boot_services = system_table.boot_services();

    let mut memmap = MemmoryMap::new(4096*4);
    memmap.GetMemoryMap(&boot_services);

    let mut simple_file_system = boot_services.get_image_file_system(image_handle).unwrap();
    let mut root_dir = simple_file_system.open_volume().unwrap();

    // メモリマップを取得
    let memmap_hadle = root_dir.open(
        cstr16!("\\memmap"),
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).unwrap();

    memmap.SaveMemoryMap(
        memmap_hadle
    );

    // カーネルファイルを読み出し
    let kernel_file_handle = root_dir.open(
        cstr16!("\\kernel.elf"),
        uefi::proto::media::file::FileMode::Read,
        FileAttribute::from_bits(0).unwrap(),
    ).unwrap();

    let mut file_info_buffer = [0; 1000];
    let file_info_handle: &mut FileInfo = 
        kernel_file_handle
            .into_regular_file()
            .unwrap()
            .get_info(&mut file_info_buffer)
            .unwrap();
    let kernel_file_size = file_info_handle.file_size();

    info!("{}", type_of(&kernel_file_size));
    info!("{}", kernel_file_size);

    let n_of_pages = (kernel_file_size + 0xfff)/0x1000;

    let kernel_base_addr = 0x100000;
    let kernel_physical_addr = 
        boot_services
            .allocate_pages(
                uefi::table::boot::AllocateType::Address(
                    kernel_base_addr as u64
                ),
                MemoryType::LOADER_DATA,
                n_of_pages as usize,
            )
            .unwrap();

    let buf: &mut [u8] = 
        unsafe {
            from_raw_parts_mut(kernel_physical_addr as *mut u8, kernel_file_size as usize)
        };

    let kernel_file_handle = root_dir.open(
        cstr16!("\\kernel.elf"),
        uefi::proto::media::file::FileMode::Read,
        FileAttribute::from_bits(0).unwrap(),
    ).unwrap();

    kernel_file_handle
        .into_regular_file()
        .unwrap()
        .read(
            buf
        );

/*
    let graphics_output: &mut GraphicsOutput = unsafe {
        boot_services
            .locate_protocol::<GraphicsOutput>()
            .unwrap()
            .get()
            .as_mut()
            .unwrap()
        };
    
    let mut mode_info: ModeInfo = 
        graphics_output
            .current_mode_info()
            .into();
    
    let mut frame_buffer = 
        graphics_output
            .frame_buffer();

    let mut frame_buffer_info = 
        FrameBufferInfo {
            fb:
                frame_buffer
                    .as_mut_ptr(),
            size:
                frame_buffer
                    .size(),
        };
*/
    
/*
    system_table
        .exit_boot_services(
            image_handle,
            &mut file_info_buffer
        );
*/

    let args =
        KernelArguments {
//            frame_buffer_info: frame_buffer_info,
//            mode_info: mode_info,
        };

    let kernel_main: extern "efiapi" fn(args: &KernelArguments) = 
        unsafe { transmute((kernel_base_addr + 24) as u64) };

    info!("{}", (kernel_base_addr + 24) as u64);

    kernel_main(&args);

    info!("Bad!!!!!!!!!!");

    Status::SUCCESS
}
