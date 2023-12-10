#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr};
use critical_section::{RawRestoreState, set_impl};

struct DummyCriticalSection;

unsafe impl critical_section::Impl for DummyCriticalSection {
    unsafe fn acquire() -> RawRestoreState {
        ()
    }

    unsafe fn release(_restore_state: RawRestoreState) {
    }
}

set_impl!(DummyCriticalSection);

use defmt::info;
use defmt_rtt as _;

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

    let count = &ebss as *const _ as usize - &sbss as *const _ as usize;
    ptr::write_bytes(sbss, 0, count);

    let count = &edata as *const _ as usize - &sdata as *const _ as usize;
    ptr::copy_nonoverlapping(sidata, sdata, count);


    const SIO_BASE: u32 = 0xd0000000;
    const SPINLOCK0_PTR: *mut u32 = (SIO_BASE + 0x100) as *mut u32;
    const SPINLOCK_COUNT: usize = 32;
    for i in 0..SPINLOCK_COUNT {
        SPINLOCK0_PTR.wrapping_add(i).write_volatile(1);
    }

    my_main()
}

#[export_name = "main"]
pub unsafe extern "C" fn my_main() -> ! {
    info!("Hello, world!");
    loop {}
}

