use bitfield::bitfield;

#[repr(C, packed)]
#[derive(Copy, Clone)]
union InterruptDescriptorAttribute {
    data: u16,
    bits: InterruptDescriptorBits,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptDescriptorBits {
    interrupt_stack_table: u16,
    reserved1: u16,                         // Use u16 or other integer type here

    desctype: DescriptorType,

    reserved2: u16,                         // Use u16 or other integer type here
    descriptor_privilege_level: u8,         // Use u8 or other integer type here
    present: bool,                          // Use bool for a 1-bit field
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct DescriptorType(u16);
    impl Debug;
    u16;
    kupper_8_bytes, set_kupper8bytes: 0;
    kidt, set_kldt: 2;
    ktssavailable, set_ktssavailable: 9;
    ktssbusy, set_ktssbusy: 11;
    kcall_gate, set_kcallgate: 12;
    kinterrupt_gate, set_kinterruptgate: 14;
    ktrap_gate, set_ktrapgate: 15;
    kguard, set_kguard: 16;
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct InterruptDescriptor {
    offset_low: u16,
    segment_selector: u16,
    attr: InterruptDescriptorAttribute,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl InterruptDescriptor {
    fn set_idt_entry(
        mut self,
        attr: InterruptDescriptorAttribute,
        offset: u64,
        segment_selector: u16,
    ) {
        self.attr               = attr;
        self.offset_low         = (offset as u16) & 0xffff;
        self.offset_middle      = ((offset >> 16) as u16) & 0xffff;
        self.offset_high        = (offset >> 32) as u32;
        self.segment_selector   = segment_selector;
    }
}