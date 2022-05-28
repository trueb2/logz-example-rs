#![cfg_attr(not(test), no_std, no_main)]
#![cfg_attr(test, allow(unused))]

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
use std::prelude::*;

use log;
#[allow(unused_imports)] // Bad warning, this implements panic_handler
#[cfg(not(test))]
use logz::fatal::panic;
pub use logz::LOGZ_LOGGER;

// #[cfg(not(test))] #[panic_handler]
// pub fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
//     todo!()
// }

#[no_mangle]
pub extern "C" fn example_foo() {
    log::trace!("Foo");
    log::debug!("Bar");
    log::info!("Fizz");
    log::warn!("Buzz");
    log::error!("Fizzle");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
