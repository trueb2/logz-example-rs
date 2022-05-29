#![cfg_attr(not(test), no_std, no_main)]
#![cfg_attr(test, allow(unused))]
#![cfg_attr(not(test), feature(arm_target_feature))]
#![feature(cfg_target_abi)]

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(target_abi = "eabihf")]
use core::arch::asm;
#[cfg(test)]
use std::prelude::*;

#[allow(unused_imports)]
use log;
#[allow(unused_imports)] // Bad warning, this implements panic_handler
#[cfg(not(test))]
use logz::fatal::panic;

#[cfg(target_abi = "eabihf")]
static mut ACC: u32 = 0;

#[cfg(target_abi = "eabihf")]
#[target_feature(enable = "dsp")]
#[no_mangle]
pub unsafe extern "C" fn example_foo() {
    log::trace!("Foo");
    log::debug!("Bar");
    log::info!("Fizz");
    log::warn!("Buzz");
    log::error!("Fizzle");
    let a: u32 = ACC;
    let b: u32 = 0x01020304;
    let c: u32;
    asm!("uqadd8 {2}, {0}, {1}", in(reg) a, in(reg) b, out(reg) c);
    ACC = c;
    log::info!("A: {:#08x} B: {:#08x} C: {:#08x}", a, b, c);
}

#[cfg(not(target_abi = "eabihf"))]
#[no_mangle]
pub unsafe extern "C" fn example_foo() {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
