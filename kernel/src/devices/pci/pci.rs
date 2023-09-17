#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(r#"
    IoOut32:
        mov dx, di
        mov eax, esi
        out dx, eax
        ret
"#);

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(r#"
    IoIn32:
        mov dx, di
        in eax, dx
        ret
"#);

#[derive(Debug, Copy, Clone)]
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
    
    pub fn add_device(
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

    pub fn scan_device(
        self,
        bus: u8,
        device: u8,
    ) -> Result<(), PciError> {
        let result = self.scan_function(
            bus,
            device,
            0
        );

        let _ = match result {
            Ok(()) => result,
            Err(e) => return Err(e),
        };

        let header_type = self.read_header_type(
            bus,
            device,
            0
        );
        if self.is_single_function_device(header_type) {
            return Ok(());
        }

        for function in 1..8 {
            let vendor_id = self.read_vendor_id(
                bus,
                device,
                function
            );

            if vendor_id == 0xffff {
                continue;
            }

            let err = self.scan_function(
                bus,
                device,
                function
            );

            let _ = match err {
                Ok(()) => err,
                Err(_e) => return err,
            };
        }

        return Ok(())
    }

    pub fn scan_bus(&self, bus: u8) -> Result<(), PciError> {
        for device in 0..32 {
            let vendor_id = self.read_vendor_id(bus, device, 0);
            if vendor_id == 0xffff {
                continue;
            }
            let result = self.scan_device(bus, device);

            match result {
                Ok(()) => continue,
                Err(e) => {
                    return Err(e)
                },
            };
        }

        Ok(())
    }

    #[allow(arithmetic_overflow)]
    pub fn scan_function(
        self,
        bus: u8,
        device: u8,
        function: u8,
    ) -> Result<(), PciError> {
        let header_type = self.read_header_type(
            bus,
            device,
            function
        );
        let error = self.add_device(
            bus,
            device,
            function,
            header_type as u8
        );

        match error {
            Ok(()) => {
                let class_code = self.read_class_code(
                    bus,
                    device,
                    function
                );
                let base = (class_code >> 24) & 0xff;
                let sub = (class_code >> 16) & 0xff;

                if (base == 0x06) && (sub == 0x04) {
                    let bus_numbers = self.read_bus_number(
                        bus,
                        device,
                        function
                    );

                    let secondly_bus = ((bus_numbers >> 8) & 0xff) as u8;
                    return self.scan_bus(secondly_bus);
                }
            },
            Err(e) => return Err(e),
        };
        error
    }

    pub fn scan_all_bus(&self) -> Result<(), PciError> {
        let header_type = self.read_header_type(0,0,0);

        if self.is_single_function_device(header_type) {
            return self.scan_bus(0);
        }

        for function in 1..8 {
            if self.read_device_id(0, 0, function) == 0xffff {
                continue;
            }
            
        }
        Ok(())
    }

    pub fn make_address(
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

    pub fn write_address(
        self,
        _address: u32
    ) {
        let _k_config_address = Pci::K_CONFIG_ADDRESS;
        unsafe {
            core::arch::asm!(
                "IoOut32(
                    _k_config_address,
                    _address
                );"
            );
        }
    }

    pub fn write_data(
        self,
        _value: u32
    ) {
        let _k_config_data = Pci::K_CONFIG_DATA;
        unsafe {
            core::arch::asm!(
                "IoOut32(
                    _k_config_data,
                    _value
                );"
            );
        }
    }

    pub fn read_data(self)-> u32 {
        let _k_config_data = Pci::K_CONFIG_DATA;
        unsafe {
            let ret = 0;
            core::arch::asm!(
                "ret = IoIn32(
                    _k_config_data
                );"
            );
            ret
        }        
    }

    pub fn read_vendor_id(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.write_address(self.make_address(bus, device, function, 0x0));
        let ret = (self.read_data() & 0xffff) as u16;
        ret
    }

    pub fn read_device_id(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.write_address(self.make_address(bus, device, function, 0x0));
        let ret = (self.read_data() >> 16) as u16;
        ret
    }

    pub fn read_header_type(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u8 {
        self.write_address(self.make_address(bus, device, function, 0x0c));
        let ret = self.read_data() >> 16;
        ret as u8
    }

    pub fn read_class_code(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u16 {
        self.write_address(self.make_address(bus, device, function, 0x08));
        let ret = (self.read_data()) as u16;
        ret
    }

    pub fn read_bus_number(
        self,
        bus: u8,
        device: u8,
        function: u8
    ) -> u8 {
        self.write_address(self.make_address(bus, device, function, 0x18));
        let ret = self.read_data();
        ret as u8
    }

    pub fn is_single_function_device(
        self,
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