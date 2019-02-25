#![no_std]
#![no_main]
#![feature(stdsimd)]

mod k20_dx256_vlh7;

use crate::k20_dx256_vlh7::{
    gpio::*,
    time::*,
    watchdog::Watchdog,
    sim::*,
};
use core::arch::arm::__NOP;

#[no_mangle]
extern fn main() {
    let (wdog,sim,mcg,osc, mut pin) = unsafe {
        (Watchdog::new(),
         Sim::new(),
         mcg::Mcg::new(),
         osc::Osc::new(),
         pin::GpioPin::new(port::PortName::C, 5))
    };

    wdog.disable();
    // Enable the crystal oscillator with 10pf of capacitance
    osc.enable(10);
    // Turn on the Port C clock gate
    sim.enable_clock(Clock::PortC);
    // Set our clocks:
    // core: 72Mhz
    // peripheral: 36MHz
    // flash: 24MHz
    unsafe {
        sim.set_dividers(1, 2, 3);
    }
    // We would also set the USB divider here if we wanted to use it.

    // Now we can start setting up the MCG for our needs.
    if let mcg::Clock::FEI(mut fei) = mcg.clock() {
        // Our 16MHz xtal is "very fast", and needs to be divided
        // by 512 to be in the acceptable FLL range.
        fei.enable_xtal(mcg::OscRange::VeryHigh);
        let fbe = fei.use_external(512);

        // PLL is 27/6 * xtal == 72MHz
        let pbe = fbe.enable_pll(27, 6);
        pbe.use_pll();
    } else {
        panic!("Somehow the clock wasn't in FEI mode");
    }

    pin.output();
    pin.high();

    loop {}
}

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
	loop{};
}

fn sleep_lol(lol: usize) {
    for _ in 0..lol {
        for _ in 0..7200 {
            unsafe {
                __NOP();
            }
        }
    }
}


extern {
    fn _stack_top();
}


// static data for mcu
#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern fn(); 2] = [
    _stack_top,
    main,
];

#[link_section = ".flashconfig"]
#[no_mangle]
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];
