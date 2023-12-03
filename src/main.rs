#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_vector;

#[no_mangle]
pub unsafe extern "C" fn reset_vector() -> ! {
    let _x = 0xbeef;
    loop {}
}