#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use cortex_m_rt::entry;
use panic_halt as _; // panic handler
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the BasedBoard it's connected to pin PC13.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for a test blinky.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(250_u32);
            led.set_low();
            delay.delay_ms(250_u32);
        }
    }

    loop {}
}
