#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

struct MemmoryMap {
    buffer_size:,
    buffer:         &mut[],
    map_size:,
    map_key:,
    descriptor_size:,
    descriptor_version:,
}

impl MemmoryMap {
    pub fn new {
        ;
    },

    pub fn GetMemoryMap(&self, system_table: SystemTable<Boot>) -> Status {
        system_table.memory_map(buffer);
    },

    pub fn GetMemoryTypeUnicode(&self) -> {
        ;
    },
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // メモリマップを取得

    // カーネルファイルを読み出し

    // カーネル起動前にブートサービスを停止

    // カーネルを起動
    
    Status::SUCCESS
}