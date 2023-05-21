#![no_std]
#![no_main]

use core::ffi::c_void;
use core::panic::PanicInfo;

#[repr(C)]
pub struct EfiTableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    _reserved: u32,
}


#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: unsafe extern "win64" fn(this: &EfiSimpleTextOutputProtocol, extended: bool) -> EfiStatus,
    pub output_string: unsafe extern "win64" fn(this: &EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    // TBD
}

#[repr(C)]
pub struct EfiSystemTable {
    pub header: EfiTableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: EfiHandle,
    _con_in: usize,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    _std_err: usize,// TBD
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct EfiHandle(*mut c_void);

#[repr(usize)]
pub enum EfiStatus {
    SUCCESS = 0,
}

#[repr(C)]
pub struct MemoryMap {
    pub buffer_size:            u64,
    pub buffer:                 void*,
    pub map_size:               u64,
    pub map_key:                u64,
    pub descriptor_size:        u64,
    pub descriptor_version:     u32,
}

#[no_mangle]
pub extern "C" fn efi_main(image: EfiHandle, st: EfiSystemTable) -> EfiStatus {

    let bt = st.boot_services();

    // メモリマップの取得(osbook_day02b)

    unsafe {
        (stdout.reset)(stdout, false);
        (stdout.output_string)(stdout, buf.as_ptr());
    }
    loop {}
    EfiStatus::SUCCESS
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}