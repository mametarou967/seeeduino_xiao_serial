#![no_std]
#![no_main]

use panic_halt as _;
use xiao_m0 as bsp;

use bsp::{entry};

use bsp::hal::{
    clock::GenericClockController,
    delay::Delay,
    ehal::blocking::delay::DelayMs,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    time::Hertz,
};

#[entry]
fn main() -> ! {
    //basic setup
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let mut serial_sercom4 = bsp::uart(
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.a7,
        pins.a6,
    );
    loop {
        delay.delay_ms(1000u16);
        for c in b"Hello World\n".iter(){
            nb::block!(serial_sercom4.write(*c)).unwrap();
        }
    }
}
