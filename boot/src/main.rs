#![no_main]
#![no_std]

#![feature(abi_efiapi)]
#![feature(error_in_core)]

extern crate alloc;

use uefi::prelude::*;
//use uefi::allocator::*;
use uefi::global_allocator::exit_boot_services;

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
    ModeInfo,
    GraphicsOutput,
};

use uefi::table::boot::*;

use uefi::Identify;

use uefi::CStr16;

use uefi::data_types::PhysicalAddress;


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
use core::arch::asm;
use core::error::Error;

use byteorder::{ByteOrder, LittleEndian};

use goblin::elf::Elf;

use lib::{KernelArguments, FrameBufferInfo, ModeInfo as OtherModeInfo};

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

/**
*   使い方
*       info!("{}", type_of(&kernel_file_size));
*
*/

fn type_of<T>(_: T) -> String{
    let a = core::any::type_name::<T>();
    return a.to_string();
  }

fn Halt() {
    loop {
        unsafe { asm!("hlt") }
    }
}

fn OpenRootDir(
    image_handle: &mut Handle,
    system_table: &mut SystemTable<Boot>,
) -> Directory {
    let mut simple_file_system = 
        system_table
            .boot_services()
            .get_image_file_system(*image_handle)
            .unwrap();
    
    let mut root_dir = 
        simple_file_system
            .open_volume()
            .unwrap();

    root_dir
}

fn SaveMemoryMap(
    mut memmap: &mut MemmoryMap,
    mut root_dir: &mut Directory,
) {
    // メモリマップを取得
    let memmap_hadle = 
        root_dir.
            open(
                cstr16!("\\memmap"),
                FileMode::CreateReadWrite,
                FileAttribute::empty(),
            )
            .unwrap();

    memmap.SaveMemoryMap(
        memmap_hadle
    );
}

fn LoadKernel<'a>(
    system_table: &'a mut SystemTable<Boot>,
    mut root_dir: &'a mut Directory,
) -> Elf<'a> {
    let kernel_file_handle = 
        root_dir.open(
            cstr16!("\\kernel.elf"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::from_bits(0).unwrap(),
            )
            .unwrap();

    let mut file_info_buffer = [0; 1000];

    let file_info_handle: &mut FileInfo = 
        kernel_file_handle
            .into_regular_file()
            .unwrap()
            .get_info(&mut file_info_buffer)
            .unwrap();

    let kernel_file_size = 
        file_info_handle
            .file_size();

    let kernel_buffer = 
        system_table
            .boot_services()
            .allocate_pool(
                MemoryType::LOADER_DATA,
                kernel_file_size as usize,
            ).unwrap();

/*
    let kernel_buffer;
    match kernel_buffer_result {
        Ok(some) => {
            kernel_buffer = some;
        },
        Err(err) => {
            Halt();
            return Err(0);
        },
    }
 */
    let kernel_buffer: &mut [u8]  = 
        unsafe {
            core::slice::from_raw_parts_mut(
                kernel_buffer,
                kernel_file_size as usize
            )
        };

    let kernel_file_handle = 
        root_dir.open(
            cstr16!("\\kernel.elf"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::from_bits(0).unwrap(),
            )
            .unwrap();


    kernel_file_handle
        .into_regular_file()
        .unwrap()
        .read(
            kernel_buffer
        );
    

    let n_of_pages = (kernel_file_size + 0xfff)/0x1000;

    let kernel_base_addr = 0x100000;
    let mut kernel_physical_addr: PhysicalAddress = 0;
    let kernel_physical_addr_result = 
        system_table
            .boot_services()
            .allocate_pages(
                uefi::table::boot::AllocateType::Address(
                    kernel_base_addr as u64
                ),
                MemoryType::LOADER_DATA,
                n_of_pages as usize,
            );

    match kernel_physical_addr_result {
        Ok(some) => {
            kernel_physical_addr = some;
        },
        Err(err) => {
            Halt();
        },
    }

    
    let buf: &mut [u8] = 
        unsafe {
            from_raw_parts_mut(
                kernel_physical_addr as *mut u8,
                kernel_file_size as usize
            )
        };

    let kernel_file_handle = 
        root_dir.open(
            cstr16!("\\kernel.elf"),
            uefi::proto::media::file::FileMode::Read,
            FileAttribute::from_bits(0).unwrap(),
        ).
        unwrap();

    kernel_file_handle
        .into_regular_file()
        .unwrap()
        .read(
            buf
        );

    let elf: Elf = Elf::parse(buf).unwrap(); 
    elf
}

#[entry]
fn main(mut image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");

    // 前処理
    let mut root_dir = 
        OpenRootDir(&mut image_handle, &mut system_table);

    let mut memmap = MemmoryMap::new(4096*4);

    SaveMemoryMap(&mut memmap, &mut root_dir);

    memmap.GetMemoryMap(&(system_table.boot_services()));

    // カーネルファイルを読み出し
    let elf = LoadKernel(&mut system_table, &mut root_dir);
    
    exit_boot_services();

    let args =
        KernelArguments {
//            frame_buffer_info: frame_buffer_info,
//            mode_info: mode_info,
        };

/*
    let buf = unsafe { core::slice::from_raw_parts((kernel_physical_addr as u64 + 24) as *mut u8, 8)};
    let kernel_main_address = LittleEndian::read_u64(&buf);
    info!("{:x}", kernel_main_address);

    /* なぜか0x100120とずれて0x101120となる */
    let kernel_main: extern "efiapi" fn(args: &KernelArguments) = 
        unsafe{ transmute(kernel_main_address) };
*/

    let kernel_main: extern "efiapi" fn(args: &KernelArguments) = 
        unsafe{ transmute(0x100120 as u64) };

    kernel_main(&args);

    info!("Bad!!!!!!!!!!");

    Status::SUCCESS
}
