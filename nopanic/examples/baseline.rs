#![feature(proc_macro_hygiene)]
#![no_main]
#![no_std]

use common as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
