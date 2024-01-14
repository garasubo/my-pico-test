use core::arch::asm;
use core::ops::Deref;
use volatile_register::{RO, RW, WO};

const SIO_BASE: usize = 0xd000_0000;

#[repr(C)]
pub struct SioRegisters {
    cpuid: RO<u32>,
    gpio_in: RO<u32>,
    gpio_hi_in: RO<u32>,
    _reserved0: u32,
    // 0x10
    gpio_out: RW<u32>,
    gpio_out_set: WO<u32>,
    gpio_out_clr: WO<u32>,
    gpio_out_xor: WO<u32>,
    // 0x20
    gpio_oe: RW<u32>,
    gpio_oe_set: WO<u32>,
    gpio_oe_clr: WO<u32>,
    gpio_oe_xor: WO<u32>,
    // 0x30
    gpio_hi_out: RW<u32>,
    gpio_hi_out_set: WO<u32>,
    gpio_hi_out_clr: WO<u32>,
    gpio_hi_out_xor: WO<u32>,
    // 0x40
    gpio_hi_oe: RW<u32>,
    gpio_hi_oe_set: WO<u32>,
    gpio_hi_oe_clr: WO<u32>,
    gpio_hi_oe_xor: WO<u32>,
    // 0x50
    fifo_st: RW<u32>,
    fifo_wr: WO<u32>,
    fifo_rd: RO<u32>,
}

impl SioRegisters {
    pub fn clear_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_clr.write(0x1 << pin);
        }
    }

    pub fn clear_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_clr.write(0x1 << pin);
        }
    }

    pub fn set_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_set.write(0x1 << pin);
        }
    }

    pub fn set_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_set.write(0x1 << pin);
        }
    }

}

pub struct Sio;

impl Sio {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_fifo_blocking(&self) -> u32 {
        while (self.fifo_st.read() & 0x1) == 0 {
            unsafe {
                asm!("wfe");
            }
        }
        self.fifo_rd.read()
    }

    pub fn drain_fifo(&self) {
        while (self.fifo_st.read() & 0x1) == 1 {
            let _ = self.fifo_rd.read();
        }
    }

    pub fn push_fifo_blocking(&self, data: u32) {
        while self.fifo_st.read() & 0x2 == 0 {}
        unsafe {
            self.fifo_wr.write(data);
            asm!("sev");
        }
    }
}

impl Deref for Sio {
    type Target = SioRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(SIO_BASE as *const SioRegisters) }
    }
}
