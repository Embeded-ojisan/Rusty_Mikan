#![no_main]
#![no_std]

pub struct ElfIdent {
    ei_class:           ElfClass,
    ei_data:            ElfData,
    ei_version:         ElfVersion,
    ei_osabi:           ElfOsAbi,
    ei_abi_version:     u8,
    ei_nident:          ,
}

pub struct Elf64_Ehdr {
    e_ident[]:      ElfIdent,
    e_type:         ,
    e_machine:      ,
    e_version:      ,
    e_entry:        ,
    e_phoff:        ,
    e_shoff:        ,
    e_flags:        ,
    e_ehsize:       ,
    e_phentsize:    ,
    e_phnum:        ,
    e_shentsize:    ,
    e_shnum:        ,
    e_shstrndx:     ,
}