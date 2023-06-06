#![no_main]
#![no_std]

#![feature(abi_efiapi)]
#![feature(error_in_core)]
#![feature(ptr_metadata)]
#![allow(stable_features)]

extern crate alloc;

use uefi::prelude::*;
//use uefi::allocator::*;
use uefi::global_allocator::exit_boot_services;

use uefi::proto::media::{
//    fs::SimpleFileSystem,
    file::*
};

use uefi::proto::console::gop::{
    ModeInfo,
    GraphicsOutput,
    PixelFormat,
};

use uefi::table::boot::{
    MemoryMapIter,
    MemoryType,
};

use uefi::data_types::PhysicalAddress;


//use alloc::vec;
//use alloc::vec::Vec;
use alloc::string::*;
use alloc::format;
//use alloc::rc::Rc;

//use core::option::Option;
//use core::ops::DerefMut;
//use core::any::type_name;
use core::mem::transmute;
use core::slice::from_raw_parts_mut;
use core::arch::asm;
//use core::error::Error;
use core::fmt::*;

use goblin::elf::*;

use log::info;

use lib::{
    KernelArguments,
    FrameBufferInfo,
    MyModeInfo,
    MemoryDescriptor,
    MemoryMap,
    MEMORY_MAP_SIZE,
};

fn GetMemoryMap<'a>(
    boot_services:      &'a BootServices,
    memmap_buffer:      &'a mut [u8]
) -> MemoryMapIter<'a> {
    let (_, memory_map_iter) = 
        boot_services
            .memory_map(memmap_buffer)
            .unwrap();

    memory_map_iter
}

fn PrintMemoryMap(
    memmap_iter: &MemoryMapIter
) {
    for (i, d) in memmap_iter.clone().enumerate() {
        let line = format!(
            "{}, {:x}, {:?}, {:08x}, {:x}, {:x}",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
    }
}

fn SaveMemoryMap<'a>(
    mut memmap_iter: &MemoryMapIter<'a>,
    mut root_dir: &mut Directory,
) {
    let mut memmap_file = root_dir
        .open(
            cstr16!("\\memorymap"),
            uefi::proto::media::file::FileMode::CreateReadWrite,
            FileAttribute::from_bits(0).unwrap(),
        )
        .unwrap()
        .into_regular_file()
        .unwrap();

        memmap_file
        .write("MemoryMap \n".as_bytes())
        .unwrap();

    for (i, d) in memmap_iter.clone().enumerate() {
        let line = format!(
            "{}, {:x}, {:?}, {:08x}, {:x}, {:x}\n",
            i,
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
        memmap_file.write(line.as_bytes()).unwrap();
    }
    memmap_file.flush().unwrap();
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
            .copy_from_slice(
                &kernel_buffer[p_offset..p_offset + p_filesz]
            );
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
        first = 
            first
                .min(
                    ph.p_vaddr as usize
                );
        last = 
            last
                .max(
                    (ph.p_vaddr + ph.p_memsz) as usize
                );
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

fn LoadKernel<'a>(
    system_table: &'a SystemTable<Boot>,
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
fn main(
    mut image_handle: Handle,
    mut system_table: SystemTable<Boot>
) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // 前処理
    info!("OpenRootDir!");
    let mut root_dir = 
        OpenRootDir(
            &mut image_handle,
            &mut system_table
        );

    // メモリマップの保存
    let memmap_size = 
        system_table
            .boot_services()
            .memory_map_size();
    
    info!("{}", memmap_size.map_size);
    let mut memmap_buffer = [0 as u8; 8000];

    info!("GetMemoryMap!");
    let memory_map_iter =
        GetMemoryMap(
            system_table.boot_services(),
            &mut memmap_buffer
        );

/*
    PrintMemoryMap(
        &memory_map_iter,
    );
*/

    info!("SaveMemoryMap!");
    SaveMemoryMap(
        &memory_map_iter,
        &mut root_dir,
    );

    // カーネルファイルを読み出し
    let elf = LoadKernel(&system_table, &mut root_dir);
    

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

    let mut mode_info: MyModeInfo 
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

    exit_boot_services();

    let mut memory_map: [MemoryDescriptor; MEMORY_MAP_SIZE] =
        [Default::default(); MEMORY_MAP_SIZE];

/*
    for (i, value) in memory_map_iter.clone().enumerate() {
        memory_map[i].memory_type       = value.ty.into();
        memory_map[i].physical_start    = value.phys_start;
        memory_map[i].virtual_start     = value.virt_start;
        memory_map[i].number_of_pages   = value.page_count;
        memory_map[i].attribute         = value.att.bits();
    }
*/

    let args =
        KernelArguments {
            frame_buffer_info: frame_buffer_info,
            mode_info: mode_info,
/*
            memory_map: MemoryMap {
                map: memory_map,
                len: memory_map_iter.len(),
            },
*/
        };

    let kernel_main: extern "efiapi" fn(args: &KernelArguments) = 
        unsafe{ transmute(elf.entry) };

    info!("Kernel_main address is {}", elf.entry);
    info!("Jump To Kernel_main!");

    kernel_main(&args);

    info!("bad!");

    Status::SUCCESS
}
