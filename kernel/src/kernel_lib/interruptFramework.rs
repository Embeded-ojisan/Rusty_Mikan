use bitfield::bitfield;

#[repr(C, packed)]
union InterruptDescriptorAttribute {
    data: u16,
    bits: InterruptDescriptorBits,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InterruptDescriptorBits {
    interrupt_stack_table: u16,
    reserved1: u16,                         // Use u16 or other integer type here

    type_: DescriptorType,

    reserved2: u16,                         // Use u16 or other integer type here
    descriptor_privilege_level: u8,         // Use u8 or other integer type here
    present: bool,                          // Use bool for a 1-bit field
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct DescriptorType(u16);
    impl Debug;
    u16;
    KUpper8Bytes, set_kupper8bytes: 0;
    KLdt, set_kldt: 2;
    KTSSAvailable, set_ktssavailable: 9;
    KTSSBusy, set_ktssbusy: 11;
    KCallGate, set_kcallgate: 12;
    KInterruptGate, set_kinterruptgate: 14;
    KTrapGate, set_ktrapgate: 15;
    KGuard, set_kguard: 16;
}

/*
impl InterruptDescriptorAttribute {
    fn MakeIDTAttr(
        mut self,
        type: DescriptorType,
        descriptor_privilege_level: u16,
        present: 
    ) -> Self {

    }
}
*/