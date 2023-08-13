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
    NullError,
    InitializeValue,
}

#[derive(Copy, Clone)]
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
    
    pub fn AddDevice(
        mut self,
        bus: u8,
        device: u8,
        function: u8,
        header_type: u8        
    ) -> Result<(), PciError> {
        if self.num_devices == self.devices.len() {
            return Err(PciError::NullError);
        }

        self.devices[self.num_devices] = Device{
            bus: bus,
            device: device,
            function: function,
            header_type: header_type
        };

        self.num_devices = self.num_devices + 1;
        Ok(())
    }

    pub fn ScanDevice(
        self,
        bus: u8,
        device: u8,
    ) -> Result<(), PciError> {
        let mut result = self.ScanFunction(
            bus,
            device,
            0
        );

        result = match result {
            Ok(()) => result,
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub fn ScanBus(&self, bus: u8) -> Result<(), PciError> {
        for device in 0..32 {
            let vendor_id = self.ReadVendorId(bus, device, 0);
            if vendor_id == 0xffff {
                continue;
            }
            let mut result = self.ScanDevice(bus, device);

            result = match result {
                Ok(()) => Ok(()),
                Err(e) => {
                    return Err(e)
                },
            };
        }

        Ok(())
    }

    #[allow(arithmetic_overflow)]
    pub fn ScanFunction(
        self,
        bus: u8,
        device: u8,
        function: u8,
    ) -> Result<(), PciError> {
        let header_type = self.ReadHeaderType(
            bus,
            device,
            function
        );
        let Error = self.AddDevice(
            bus,
            device,
            function,
            header_type as u8
        );

        match Error {
            Ok(()) => {
                let class_code = self.ReadClassCode(
                    bus,
                    device,
                    function
                );
                let base = (class_code >> 24) & 0xff;
                let sub = (class_code >> 16) & 0xff;

                if (base == 0x06) && (sub == 0x04) {
                    let bus_numbers = self.ReadBusNumber(
                        bus,
                        device,
                        function
                    );

                    let secondly_bus = ((bus_numbers >> 8) & 0xff) as u8;
                    return self.ScanBus(secondly_bus);
                }
            },
            Err(e) => return Err(e),
        };
        Error
    }

/*
    pub fn ScanAllBus() -> Result<(), PciError> {
        let mut num_device = 0;
        let header_type = Self::ReadHeaderType();

        if Self::IsSingleFunctionDevice(header_type) {
            return Self::ScanBus(0);
        }

        for i in 1..8 {
            if Self::ReadDeviceId(0, 0, function) == 0xffff {
                continue;
            }
            
        }
        Ok(())
    }
*/
    pub fn MakeAddress(
        self,
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
        self,
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
        self,
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

    pub fn ReadData(self)-> u32 {
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
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.writeAddress(self.MakeAddress(bus, device, function, 0x0));
        let ret = (self.ReadData() & 0xffff) as u16;
        ret
    }

    pub fn ReadDeviceId(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.writeAddress(self.MakeAddress(bus, device, function, 0x0));
        let ret = (self.ReadData() >> 16) as u16;
        ret
    }

    pub fn ReadHeaderType(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u8 {
        self.writeAddress(self.MakeAddress(bus, device, function, 0x0c));
        let ret = self.ReadData() >> 16;
        ret as u8
    }

    pub fn ReadClassCode(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.writeAddress(self.MakeAddress(bus, device, function, 0x08));
        let ret = (self.ReadData()) as u16;
        ret
    }

    pub fn ReadBusNumber(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u8 {
        self.writeAddress(self.MakeAddress(bus, device, function, 0x18));
        let ret = self.ReadData();
        ret as u8
    }

    pub fn IsSingleFunctionDevice(
        header_type: u8
    ) -> bool {
        (header_type & 0x80) == 0
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