use lib::allocator::*;

use crate::kernel_lib::interrupt_framework::*;

static mut IDT: InterrutpDescriptionTable;

pub struct InterrutpDescriptionTable {
    InterrutpDescriptionTable_body: Vec<InterruptDescriptor>
}

impl InterrutpDescriptionTable {
    const IDT_NUM: u8 = 256;
}