#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::allocator::*;

use uefi::proto::media::{
    fs::SimpleFileSystem,
    file::*
};

use uefi::proto::device_path::{
    text::DevicePathFromText,
    build::DevicePathBuilder,
};

use uefi::table::boot::{
    SearchType,
    ScopedProtocol,
};

use uefi::Identify;

use uefi::CStr16;


use alloc::vec;
use alloc::vec::Vec;
use log::info;

use core::option::Option;
use core::ops::DerefMut;


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
    mDevicePathFromText:      ScopedProtocol<'a, DevicePathFromText>,
}

impl<'a> EfiProtocols<'a> {
    pub fn new(
        boot_services: &'a uefi::prelude::BootServices,
    ) -> Self {

        EfiProtocols {
            mSimpleFileSystem: 
                boot_services
                    .open_protocol_exclusive::<SimpleFileSystem>(
                        boot_services.image_handle()
                ).unwrap(),

            mDevicePathFromText:
                boot_services
                    .open_protocol_exclusive::<DevicePathFromText>(
                        boot_services.image_handle()
                ).unwrap(),
        }
    }
}

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // 前処理
    let boot_services = system_table.boot_services();

    let mut memmap = MemmoryMap::new(4096*4);
    memmap.GetMemoryMap(&boot_services);

    let mut efiprotocols = EfiProtocols::new(&boot_services);

    let mut root_dir = 
        (*(efiprotocols.mSimpleFileSystem.deref_mut()))
            .open_volume()
            .unwrap();

    // メモリマップを取得
    let memmap_hadle = root_dir.open(
        cstr16!("memmap"),
        FileMode::CreateReadWrite,
        FileAttribute::empty(),
    ).unwrap();

    memmap.SaveMemoryMap(
        memmap_hadle
    );

    // カーネルファイルを読み出し
    let kernel_file_handle = root_dir.open(
        cstr16!("kernel.elf"),
        FileMode::Read,
        FileAttribute::READ_ONLY,
    ).unwrap();

    if let Some(mut regular) = kernel_file_handle.into_regular_file() {
        let mut file_info_buffer = [0; 1000];
        let file_info_handle: &mut FileInfo = 
            regular
                .get_info(&mut file_info_buffer)
                .unwrap();
        let kernel_file_size = file_info_handle.file_size();
    }

    // カーネル起動前にブートサービスを停止

    // カーネルを起動
    
    Status::SUCCESS
}
