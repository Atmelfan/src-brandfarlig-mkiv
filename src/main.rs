#![no_std]
#![no_main]

/* Platform */
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


/* Miscellaneous */
use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f4::stm32f405;
use stm32f4::stm32f405::{interrupt, Interrupt, NVIC};

/* Devicetree */
use static_dt_rs::{DeviceTree, Token};
use cortex_m::peripheral::syst::SystClkSource;

mod platform;

const FDT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tree.dtb"));

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let mut peripherals = stm32f405::Peripherals::take().unwrap();
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::EXTI0);

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();
    syst.enable_interrupt();

    let gpioa = &peripherals.GPIOA;
    let spi1 = &peripherals.SPI1;

    gpioa.otyper.write(|x| x.ot0().push_pull());
    gpioa.moder.write(|x| x.moder0().output());
    gpioa.odr.write(|x| x.odr0().high());

    let mut fdt = FDT;

    let dt = DeviceTree::use_buffer(fdt).unwrap();

    let bht_root = dt.root().get_node(b"select@root").unwrap();


    loop {
        gpioa.odr.modify(|r, w| w.odr0().bit(!r.odr0().bit()));
    }
}

#[interrupt]
fn EXTI0() {

}
