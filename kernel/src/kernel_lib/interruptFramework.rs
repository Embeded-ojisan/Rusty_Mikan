#[repr(C, packed)]
union InterruptDescriptorAttribute {
    data: u16,
    bits: InterruptDescriptorBits,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct InterruptDescriptorBits {
    interrupt_stack_table: u16,
    reserved1: u16,
    type_: DescriptorType,
    reserved2: u16,
    descriptor_privilege_level: u16,
    present: u16,
}


#[repr(u16)]
#[derive(Debug, Copy, Clone)]
enum DescriptorType { 
    Type1 = 1,
    Type2 = 2,
    Type3 = 3,
    // Add more enum variants as needed
}