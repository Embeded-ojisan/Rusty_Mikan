#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

struct MemmoryMap {
    ,
}

impl MemmoryMap {
    pub fn GetMemoryMap(&self, image_handle: Handle) -> Status {
        ;
    }
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