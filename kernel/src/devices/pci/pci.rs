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

#[derive(Debug)]
pub enum PciError {
    Error,
}

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

    pub fn MakeAddress(
        bus: u8,
        device: u8,
        function: u8,
        reg_addr: u8
    ) -> u32 {
        let shl = |x, bits| (x as u32) << bits;

        shl(1, 31)
        | shl(bus, 16)
        | shl(device, 11)
        | shl(function, 8)
        | ((reg_addr as u32) & 0xfc)
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

    pub fn ReadVendorId(
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        Self::writeAddress(Self::MakeAddress(bus, device, function, 0x0));
        let ret = (Self::ReadData() & 0xffff) as u16;
        ret
    }

    pub fn ReadDeviceId(
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        Self::writeAddress(Self::MakeAddress(bus, device, function, 0x0));
        let ret = (Self::ReadData() >> 16) as u16;
        ret
    }

    pub fn ReadHeaderType(
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        Self::writeAddress(Self::MakeAddress(bus, device, function, 0x0c));
        let ret = (Self::ReadData() >> 16) as u16;
        ret
    }

    pub fn ReadClassCode(
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        Self::writeAddress(Self::MakeAddress(bus, device, function, 0x08));
        let ret = (Self::ReadData()) as u16;
        ret
    }

    pub fn ReadBusNumber(
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        Self::writeAddress(Self::MakeAddress(bus, device, function, 0x18));
        let ret = (Self::ReadData()) as u16;
        ret
    }

    pub fn IsSingleFunctionDevice(
        header_type: u8
    ) -> bool {
        (header_type & 0x80) == 0
    }

    pub fn ScanAllBus() -> Result<(), PciError> {
        Ok(())
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