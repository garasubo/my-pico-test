use core::ops::Deref;
use volatile_register::{RO, RW};

const CLOCK_BASE: usize = 0x4000_8000;

#[repr(C)]
pub struct ClockRegisters {
    gpout0_ctrl: RW<u32>,
    gpout0_div: RW<u32>,
    gpout0_selected: RW<u32>,
    gpout1_ctrl: RW<u32>,
    // 0x10
    gpout1_div: RW<u32>,
    gpout1_selected: RW<u32>,
    gpout2_ctrl: RW<u32>,
    gpout2_div: RW<u32>,
    // 0x20
    gpout2_selected: RW<u32>,
    gpout3_ctrl: RW<u32>,
    gpout3_div: RW<u32>,
    gpout3_selected: RW<u32>,
    // 0x30
    ref_ctrl: RW<u32>,
    ref_div: RW<u32>,
    ref_selected: RW<u32>,
    pub(crate) sys_ctrl: RW<u32>,
    // 0x40
    sys_div: RW<u32>,
    sys_selected: RW<u32>,
    peri_ctrl: RW<u32>,
    // TBD
}

pub struct Clock;

impl Deref for Clock {
    type Target = ClockRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(CLOCK_BASE as *const ClockRegisters) }
    }
}

const XOSC_BASE: usize = 0x4002_4000;

pub struct XoscRegisters {
    ctrl: RW<u32>,
    status: RW<u32>,
    dormant: RW<u32>,
    startup: RW<u32>,
    _reserved0: [u32; 3],
    // 0x1c
    count: RW<u32>,
}

pub struct Xosc;

impl Deref for Xosc {
    type Target = XoscRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(XOSC_BASE as *const XoscRegisters) }
    }
}

impl Xosc {
    pub fn new() -> Self {
        Self {}
    }
    pub fn init(&self) {
        let clock = Clock {};
        unsafe {
            clock.sys_ctrl.write(0); // enable uart clock
        }
        unsafe {
            self.startup.write(47); // TODO: なにこれ
            self.ctrl.write(0xfabaa0); // enable xosc, 1-15MHz
        }
        // wait for xosc stable
        while self.status.read() & 1 << 31 == 0 {}
        unsafe {
            // select xosc as source
            clock.ref_ctrl.write(0x2);
            // select clk_ref as source
            clock.sys_ctrl.write(0x0);
            // enable / xosc_clksrc
            clock.peri_ctrl.write((1 << 11) | (4 << 5));
        }
    }
}
