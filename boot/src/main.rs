#![no_main]
#![no_std]

use core::{mem, slice};

use goblin::elf;

use log::info;

use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use uefi::table::cfg::ACPI_GUID;
use uefi::table::Runtime;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");
    system_table.boot_services().stall(10_000_000);

    // カーネルのイメージの取り出し

    // elfファイルの取り出し
    
    Status::SUCCESS
}