#![feature(global_asm, asm)]

#[cfg(target_arch = "x86_64")]
#[link_section = ".text.boot"]
core::arch::global_asm!(r#"
    IoOut32:
        mov dx, di
        mov eax, esi
        out dx, eax
        ret
"#);

#[cfg(target_arch = "x86_64")]
#[link_section = ".text.boot"]
core::arch::global_asm!(r#"
    IoIn32:
        mov dx, di
        in eax, dx
        ret
"#);


pub struct Pci {
    devices: [Device; Pci::MAX_DEVICE_NUM],
    num_devices: usize,
}

impl Pci {
    /* CONFIG_ADDRESS レジスタの IO ポートアドレス */
    const K_CONFIG_ADDRESS: u16 = 0x0cf8;
    
    /** @brief CONFIG_DATA レジスタの IO ポートアドレス */
    const K_CONFIG_DATA: u16 = 0x0cfc;

    const MAX_DEVICE_NUM: usize = 32;

    pub fn new() -> Self {
        Pci {
            devices: [
                Device {
                    bus: 0,
                    device: 0,
                    function: 0,
                    header_type: 0,
                };
                Pci::MAX_DEVICE_NUM
            ],
            num_devices: Pci::MAX_DEVICE_NUM,
        }
    }

    pub fn writeAddress(
        address: u32
    ) {
        let kConfigAddress = Pci::K_CONFIG_ADDRESS;
        unsafe {
            core::arch::asm!(
                "IoOut32(
                    kConfigAddress,
                    address
                );"
            );
        }
    }

    pub fn writeData(
        value: u32
    ) {
        let kConfigData = Pci::K_CONFIG_DATA;
        unsafe {
            core::arch::asm!(
                "IoOut32(
                    kConfigData,
                    value
                );"
            );
        }
    }

    pub fn ReadData()-> u32 {
        let kConfigData = Pci::K_CONFIG_DATA;
        unsafe {
            let mut ret = 0;
            core::arch::asm!(
                "ret = IoIn32(
                    kConfigData
                );"
            );
            ret
        }        
    }
}

#[derive(Copy)]
#[derive(Clone)]
struct Device {
    bus: u8,
    device: u8,
    function: u8,
    header_type: u8,
}