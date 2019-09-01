#![no_std]
#![no_main]

/* Platform */
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
//extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger


/* Miscellaneous */
use cortex_m::asm;
use cortex_m_rt::entry;

/* Devicetree */
use static_dt_rs::{DeviceTree, Token};

mod platform;

#[link_section = ".CCRAM"]
const FDT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tree.dtb"));

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let mut fdt = FDT;

    let dt = DeviceTree::use_buffer(fdt).unwrap();

    let bht_root = dt.root().get_node(b"select@root").unwrap();


    loop {
        // your code goes here
    }
}
