use core::ops::Deref;
use volatile_register::{RO, RW};
use crate::clock::{Clock, Xosc};
use crate::gpio::{Gpio, Reset, RESET_BASE};

const UART0_BASE: usize = 0x4003_4000;

#[repr(C)]
pub struct UartRegister {
    dr: RW<u32>,
    rsr: RW<u32>,
    _reserved0: [u32; 4],
    // 0x18
    fr: RO<u32>,
    _reserved1: u32,
    ilpr: RW<u32>,
    ibrd: RW<u32>,
    fbrd: RW<u32>,
    lcr_h: RW<u32>,
    cr: RW<u32>,
    ifls: RW<u32>,
    imsc: RW<u32>,
    ris: RW<u32>,
    mis: RW<u32>,
    icr: RW<u32>,
    dmacr: RW<u32>,
}
pub struct Uart0;

impl Deref for Uart0 {
    type Target = UartRegister;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(UART0_BASE as *const UartRegister) }
    }
}

impl Uart0 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self, xosc: &Xosc, gpio: &Gpio) {
        // Baud rate = 115200

        // clock init
        xosc.init();

        // Uart reset
        let reset = unsafe { &mut *(RESET_BASE as *mut Reset) };
        // Bit 22: UART0
        unsafe { reset.reset.modify(|r| r & !(1 << 22)); }
        while reset.reset_done.read() & (1 << 22) == 0 {}

        unsafe {
            self.ibrd.write(6);
            self.fbrd.write(33);
            self.lcr_h.write(0x70); // 8bit, no parity, 1 stop bit, FIFO disable
            self.cr.write((1 << 9) | (1 << 8) | (1 << 0)); // enable uart, tx, rx
        }

        gpio.set_pin_function(0, 0x2);
        gpio.set_pin_function(1, 0x2);
    }

    pub fn putc(&self, c: u8) {
        // check if transmit FIFO is full
        while self.fr.read() & (1 << 5) != 0 {}
        unsafe {
            self.dr.write(c as u32);
        }
    }

    pub fn getc(&self) -> u8 {
        // check if receive FIFO is empty
        while self.fr.read() & (1 << 4) != 0 {}
        unsafe {
            self.dr.read() as u8
        }
    }
}