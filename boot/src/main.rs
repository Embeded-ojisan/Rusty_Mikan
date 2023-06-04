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

use goblin::elf::*;

use lib::{
    KernelArguments,
    FrameBufferInfo,
    ModeInfo as OtherModeInfo,
    MemoryDescriptor,
    MemoryMap,
    MEMORY_MAP_SIZE,
};

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

        // headerを用意

        // headerをhandleに記録

        // メモリマップの各要素を書き込み
        

        Status::SUCCESS
    }
    pub fn ReturnMemmoryMapIterator(
        &mut self,
    ) -> &mut MemoryMapIter {
        &mut (*self).entries()
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

fn CopyLoadSegments<'a>(
    elf:                &'a Elf<'_>,
    kernel_buffer:      &[u8]
) {
    for ph in elf.program_headers.iter() {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        let p_offset = ph.p_offset as usize;
        let p_filesz = ph.p_filesz as usize;
        let p_memsz = ph.p_memsz as usize;
        let dest = 
            unsafe { 
                from_raw_parts_mut(
                    ph.p_vaddr as *mut u8, p_memsz
                )
            };
        dest[..p_filesz]
            .copy_from_slice(&kernel_buffer[p_offset..p_offset + p_filesz]);
        dest[p_filesz..]
            .fill(0);
    }
}

fn CalcLoadAddressRange<'a>(
    elf:    &'a Elf<'_>,
    mut first: usize,
    mut last:  usize,
) -> (usize, usize) {
    first = usize::MAX;
    last = 0;
    for ph in elf.program_headers.iter() {
        if ph.p_type != program_header::PT_LOAD {
            continue;
        }
        first = first.min((ph.p_vaddr as usize));
        last = last.max(((ph.p_vaddr + ph.p_memsz) as usize));
    }
    (first, last)
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

    let elf: Elf = Elf::parse(kernel_buffer).unwrap(); 

    let mut kernel_first_address=0;
    let mut kernel_last_address=0;
    let (kernel_first_address, kernel_last_address) =
        CalcLoadAddressRange(
            &elf,
            kernel_first_address,
            kernel_last_address,
        );

    let num_pages = 
        (kernel_last_address - kernel_first_address)/0x1000;

    let mut kernel_physical_addr: PhysicalAddress = 0;
    let kernel_physical_addr_result = 
        system_table
            .boot_services()
            .allocate_pages(
                uefi::table::boot::AllocateType::Address(
                    kernel_first_address as u64
                ),
                MemoryType::LOADER_DATA,
                num_pages as usize,
            );

    match kernel_physical_addr_result {
        Ok(some) => {
            kernel_physical_addr = some;
        },
        Err(err) => {
            Halt();
        },
    }

    CopyLoadSegments(&elf, kernel_buffer);

    elf
}

#[entry]
fn main(mut image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    info!("Hello world!");

    // 前処理
    let mut root_dir = 
        OpenRootDir(&mut image_handle, &mut system_table);

    // メモリマップの保存
    let mut memmap = MemmoryMap::new(4096*4);
    SaveMemoryMap(&mut memmap, &mut root_dir);
    memmap.GetMemoryMap(&(system_table.boot_services()));

    // 
    let gop: &mut GraphicsOutput = 
        unsafe {
            system_table
                .boot_services()
                .locate_protocol::<GraphicsOutput>()
                .unwrap()
                .get()
                .as_mut()
                .unwrap()
        };

    let mut mode_info: ModeInfo 
        = gop
            .current_mode_info()
            .into();

    let mut frame_buffer =
        gop
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
    
    let mut memory_map: [MemoryDescriptor; MEMORY_MAP_SIZE]
        = [Default::default(); MEMORY_MAP_SIZE];

    for (i, value) in memory_map_iter.clone().enumerate() {
        memory_map[i].memory_type = value.ty.into();
        memory_map[i].physical_start = value.phys_start;
        memory_map[i].virtual_start = value.virt_start;
        memory_map[i].number_of_pages = value.page_count;
        memory_map[i].attribute = value.att.bits();
    }
    

    // カーネルファイルを読み出し
    let elf = LoadKernel(&mut system_table, &mut root_dir);

    exit_boot_services();

    let args =
        KernelArguments {
//            frame_buffer_info: frame_buffer_info,
//            mode_info: mode_info,
        };

    let kernel_main: extern "efiapi" fn(args: &KernelArguments) = 
        unsafe{ transmute(elf.entry) };


    kernel_main(&args);

    info!("Bad!!!!!!!!!!");

    Status::SUCCESS
}
