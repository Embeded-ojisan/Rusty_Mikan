#![no_main]
#![no_std]
//#![feature(alloc_error_handler)]

extern crate alloc;

use uefi::prelude::*;
use uefi::allocator::*;

use alloc::vec::Vec;
use log::info;

struct MemmoryMap<'a> {
    buffer_size: usize,
    buffer: Option<Vec<&'a u8>>,
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: usize,
}

impl<'a> MemmoryMap<'a> {
    pub fn new(inp_buffer_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(inp_buffer_size);
        buffer.resize(inp_buffer_size, &0u8);
        MemmoryMap {
            buffer_size: inp_buffer_size,
            buffer: Some(buffer),
            map_size: 0,
            map_key: 0,
            descriptor_size: 0,
            descriptor_version: 0,
        }
    }
/*
    pub fn GetMemoryMap(&self, system_table: SystemTable<Boot>) -> Status {
        system_table.memory_map(buffer);
    },

    pub fn GetMemoryTypeUnicode(&self) ->Self {
        ;
    },
*/
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // メモリマップを取得
    let mut memmap = MemmoryMap::new(4096*4);

    // カーネルファイルを読み出し

    // カーネル起動前にブートサービスを停止

    // カーネルを起動
    
    Status::SUCCESS
}