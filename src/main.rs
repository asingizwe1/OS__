// main.rs
#![no_std]

use core::panic::PanicInfo;
//panic handler function to handle a panic

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    //reason for this error is that the println macro is part of the standard library, which we no longer include
    //println!("Hello, world!");
    //we need to define the panic handler our selves
}
