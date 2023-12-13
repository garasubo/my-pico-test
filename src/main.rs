#![no_std]
#![no_main]

mod gpio;
mod systick;

use core::{panic::PanicInfo, ptr};
use crate::gpio::Gpio;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
#[link_section = ".vector_table.reset_vector"]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let sbss = &mut _sbss as *mut u8;
    let ebss = &mut _ebss as *mut u8;
    let sdata = &mut _sdata as *mut u8;
    let edata = &mut _edata as *mut u8;
    let sidata = &_sidata as *const u8;

    let count = ebss as usize - sbss as usize;
    ptr::write_bytes(sbss, 0, count);

    let count = edata as usize - sdata as usize;
    ptr::copy_nonoverlapping(sidata, sdata, count);


    my_main()
}

pub fn my_main() -> ! {
    let gpio = Gpio::new();
    gpio.wait_gpio_reset_done();
    gpio.set_output_enable(6);
    gpio.set_high(6);

    systick::init(1000 * 1000);
    loop {
        while !systick::check_counted() {}
        gpio.set_low(6);
        while !systick::check_counted() {}
        gpio.set_high(6);
    }
}
