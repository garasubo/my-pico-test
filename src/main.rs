#![no_std]
#![no_main]

mod clock;
mod cs;
mod gpio;
mod systick;
mod uart;
mod sio;

use core::arch::asm;
use crate::gpio::Gpio;
use core::panic::PanicInfo;
use core::ptr;
#[cfg(feature="probe-run")]
use defmt_rtt as _; // global logger
#[cfg(feature="probe-run")]
use panic_probe as _;

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[cfg(feature="probe-run")]
macro_rules! dprintln {
    () => {
        defmt::println!()
    };
    ($s:expr) => {
        defmt::println!($s)
    };
    ($s:expr, $($tt:tt)*) => {
        defmt::println!($s, $($tt)*)
    };
}

#[cfg(not(feature="probe-run"))]
macro_rules! dprintln {
    () => {};
    ($s:expr) => {};
    ($s:expr, $($tt:tt)*) => {};
}

#[cfg(not(feature="probe-run"))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature="probe-run")]
#[defmt::panic_handler]
fn panic() -> ! {
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
    Vector {
        handler: UsageFault,
    },
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

static mut CORE1_STACK: [u8; 1024] = [0; 1024];

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

    dprintln!("hello");

    boot_core1();
    dprintln!("boot core1");

    let xosc = clock::Xosc::new();
    let uart = uart::Uart0::new();
    uart.init(&xosc, &gpio);


    systick::init(1000 * 1000);
    loop {
        while !systick::check_counted() {}
        for c in b"hello\n" {
            uart.putc(*c);
        }
    }
}

pub extern "C" fn my_main_core1() -> ! {
    let gpio = Gpio::new();
    gpio.set_low(6);

    loop {

    }
}

fn boot_core1() {
    extern "C" {
        static mut _reset_vector: u8;
    }
    let reset_vector = unsafe { &_reset_vector as *const u8 as usize };
    let sp = unsafe { CORE1_STACK.as_ptr() as usize + CORE1_STACK.len() };
    // 0x1: Thumb mode
    let entry_point = my_main_core1 as *const () as usize | 0x1;
    let cmds: [usize; 6] = [0, 0, 1, reset_vector, sp, entry_point];
    let sio = sio::Sio::new();

    let mut iter = cmds.into_iter();
    let mut current = iter.next();
    while let Some(cmd) = current {
        // always drain the READ FIFO (from core 1) before sending a 0
        if cmd == 0 {
            dprintln!("drain fifo");
            sio.drain_fifo();
            // execute a SEV as core 1 may be waiting for FIFO space
            unsafe {
                asm!("sev");
            }
        }

        dprintln!("write fifo: {}", cmd);
        // write 32 bit value to write FIFO
        sio.push_fifo_blocking(cmd as u32);
        dprintln!("reading fifo");
        // read 32 bit value from read FIFO once available
        let response = sio.read_fifo_blocking();
        if response == cmd as u32 {
            dprintln!("response: {}", response);
            current = iter.next();
       } else {
            iter = cmds.into_iter();
            current = iter.next();
        }
    }
}
