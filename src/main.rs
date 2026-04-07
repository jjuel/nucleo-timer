#![no_std]
#![no_main]

use stm32h7xx_hal::{pac, prelude::*};
use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.ldo().freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let mut led = gpioe.pe1.into_push_pull_output();

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let but = gpioc.pc13.into_pull_down_input();

    loop {
        if but.is_high() {
           led.set_high();
        } else {
           led.set_low();
        }
    }
}
