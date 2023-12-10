use core::ptr::{read_volatile, write_volatile};

const CSR_ADDR: usize = 0xE000_E010;
const RVR_ADDR: usize = 0xE000_E014;
const CVR_ADDR: usize = 0xE000_E018;

pub fn init(reload_value: u32) {
    unsafe {
        write_volatile(CVR_ADDR as *mut u32, 0);
        write_volatile(RVR_ADDR as *mut u32, reload_value);
        write_volatile(CSR_ADDR as *mut u32, 0x1);
    }
}

pub fn check_counted() -> bool {
    unsafe { read_volatile(CSR_ADDR as *const u32) & (1 << 16) > 0 }
}