#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;
use uefi::allocator::*;

use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::device_path::{
    text::DevicePathToText,
    build::DevicePathBuilder,
};
use uefi::table::boot::{
    SearchType,
    ScopedProtocol,
};
use uefi::Identify;


use alloc::vec;
use alloc::vec::Vec;
use log::info;

use core::option::Option;

/*
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::SearchType;
use uefi::{Identify, Result};
*/

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
        system_table: SystemTable<Boot>,
    ) -> Status {
        match self.buffer.as_mut() {
            Some(buf) => {
                system_table
                    .boot_services()
                    .memory_map(
                        buf.as_mut_slice()
                    );
                Status::SUCCESS
            },
            None => Status::BUFFER_TOO_SMALL,
        }
    }
/*
    pub fn SaveMemmoryMap(
        &self, 
        system_table: SystemTable<Boot>
    )
        -> Status {
            ;
    }
*/
}


struct EfiProtocols<'a> {
    mSimpleFileSystem:      ScopedProtocol<'a, SimpleFileSystem>,
    mDevicePathToText:      ScopedProtocol<'a, DevicePathToText>,
//    mDevicePathBuilder:     ScopedProtocol<'a, DevicePathBuilder>,
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

            mDevicePathToText:
                boot_services
                    .open_protocol_exclusive::<DevicePathToText>(
                        boot_services.image_handle()
                ).unwrap(),

/*
            mDevicePathBuilder:
                boot_services
                    .open_protocol_exclusive::<DevicePathBuilder>(
                        boot_services.image_handle()
                ).unwrap(),
*/

        }
    }
}
/*
fn OpenRootDir(
    image_handle: Handle,
    root: uefi::file
) -> Status {
    uefi


    Status::SUCCESS
}
*/

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // 前処理
    let mut memmap = MemmoryMap::new(4096*4);
    memmap.GetMemoryMap(system_table);

    // メモリマップを取得

    // カーネルファイルを読み出し

    // カーネル起動前にブートサービスを停止

    // カーネルを起動
    
    Status::SUCCESS
}
