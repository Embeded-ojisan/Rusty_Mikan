#![no_main]
#![no_std]
//#![feature(alloc_error_handler)]

extern crate alloc;

use uefi::prelude::*;
use uefi::allocator::*;

use alloc::vec;
use alloc::vec::Vec;
use log::info;

use core::option::Option;

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
    pub fn GetMemoryTypeUnicode(&self) ->Self {
        ;
    }
*/
/*
    pub fn SaveMemmoryMap(&self, system_table: SystemTable<Boot>)
        -> SystemTable<Boot> {
            ;
    }
*/
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // メモリマップを取得
    let mut memmap = MemmoryMap::new(4096*4);
    memmap.GetMemoryMap(system_table);

    // カーネルファイルを読み出し

    // カーネル起動前にブートサービスを停止

    // カーネルを起動
    
    Status::SUCCESS
}