core::arch::global_asm!(include_str!("./pci.asm"));

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
}

#[derive(Copy)]
#[derive(Clone)]
struct Device {
    bus: u8,
    device: u8,
    function: u8,
    header_type: u8,
}