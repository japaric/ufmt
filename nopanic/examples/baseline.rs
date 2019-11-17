// used as a reference in the `size` output (see ci/script.sh)

#![no_main]
#![no_std]

use common as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
