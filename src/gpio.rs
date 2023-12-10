use core::ops::Deref;
use volatile_register::{RO, RW, WO};

const SIO_BASE: usize = 0xd000_0000;

#[repr(C)]
struct SioRegisters {
    cpuid: RO<u32>,
    gpio_in: RO<u32>,
    gpio_hi_in: RO<u32>,
    gpio_out: RW<u32>,
    gpio_out_set: WO<u32>,
    gpio_out_clr: WO<u32>,
    gpio_out_xor: WO<u32>,
    gpio_oe: RW<u32>,
    gpio_oe_set: WO<u32>,
    gpio_oe_clr: WO<u32>,
    gpio_oe_xor: WO<u32>,
    // TBD
}

impl SioRegisters {
    fn clear_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_clr.write(0x1 << pin);
        }
    }

    fn clear_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_clr.write(0x1 << pin);
        }
    }

    fn set_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_set.write(0x1 << pin);
        }
    }

    fn set_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_set.write(0x1 << pin);
        }
    }
}

struct Sio;

impl Deref for Sio {
    type Target = SioRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(SIO_BASE as *const SioRegisters) }
    }
}

const IO_BANK0_BASE: usize = 0x4001_4000;

#[repr(C)]
pub struct GpioPin {
    status: RW<u32>,
    ctrl: RW<u32>,
}

#[repr(C)]
pub struct IoBankRegisters {
    pub pins: [GpioPin; 30],
    // 0x0f0
    pub intr: RW<u32>,
    // 0x100
    pub proc0_inte: [RW<u32>; 4],
    pub proc0_intf: [RW<u32>; 4],
    pub proc0_ints: [RW<u32>; 4],
    // 0x130
    pub proc1_inte: [RW<u32>; 4],
    pub proc1_intf: [RW<u32>; 4],
    pub proc1_ints: [RW<u32>; 4],
    // 0x160
    pub dormant_wake_inte: [RW<u32>; 4],
    pub dormant_wake_intf: [RW<u32>; 4],
    pub dormant_wake_ints: [RW<u32>; 4],
}

struct IoBank0;

impl Deref for IoBank0 {
    type Target = IoBankRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(IO_BANK0_BASE as *const IoBankRegisters) }
    }
}

const PADS_BANK0_BASE: usize = 0x4001_c000;

#[repr(C)]
pub struct PadsBankRegisters {
    pub voltage_select: RW<u32>,
    pub gpio: [RW<u32>; 30],
    pub swclk: RW<u32>,
    // 0x80
    pub swd: RW<u32>,
}

struct PadsBank0;

impl Deref for PadsBank0 {
    type Target = PadsBankRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(PADS_BANK0_BASE as *const PadsBankRegisters) }
    }
}


pub struct Gpio {
    sio: Sio,
    io_bank0: IoBank0,
    pads_bank0: PadsBank0,
}

impl Gpio {
    pub fn new() -> Self {
        let sio = Sio;
        let io_bank0 = IoBank0;
        let pads_bank0 = PadsBank0;
        Self {
            sio,
            io_bank0,
            pads_bank0,
        }
    }

    pub fn wait_gpio_reset_done(&self) {
        let reset = unsafe { &mut *(RESET_BASE as *mut Reset) };
        // Bit 5: IO_BANK0
        unsafe { reset.reset.modify(|r| r & !(1 << 5)); }
        while reset.reset_done.read() & (1 << 5) == 0 {}
        // Bit 8: PADS_BANK0
        unsafe { reset.reset.modify(|r| r & !(1 << 8)); }
        while reset.reset_done.read() & (1 << 8) == 0 {}
    }

    pub fn set_output_enable(&self, pin: usize) {
        unsafe {
            // clear output
            self.sio.clear_gpio_oe(pin);
            self.sio.clear_gpio_out(pin);

            // function select (select SIO)
            self.io_bank0.pins[pin].ctrl.write(0x5);
            // clear OD (output disable) bit
            self.pads_bank0.gpio[pin].modify(|v| v & !(1 << 7));

            // enable output
            self.sio.set_gpio_oe(pin)
        }
    }

    pub fn set_high(&self, pin: usize) {
        unsafe {
            self.sio.set_gpio_out(pin);
        }
    }

    pub fn set_low(&self, pin: usize) {
        unsafe {
            self.sio.clear_gpio_out(pin);
        }
    }
}

const RESET_BASE: usize = 0x4000_c000;

#[repr(C)]
pub struct Reset {
    pub reset: RW<u32>,
    pub wdsel: RW<u32>,
    pub reset_done: RO<u32>,
}