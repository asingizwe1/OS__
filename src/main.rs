// main.rs
#![no_std]

use core::panic::PanicInfo;
//panic handler function to handle a panic
// PanicInfo parameter contains the file and line where the panic happened and the optional panic message.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
//Rust uses stack unwinding by default to run the destructors of all live stack variables in case of a panic. This ensures that all used memory is freed and allows the parent thread to catch the panic and continue execution.

//runtime needs to be called before main MAIN isnt the main entry point else cargo build will "error: using `fn main` requires the standard library"
//main doesnt make sense without an underlying runtime that calls it hence its removed
//we are overwriting the OS entry point with our own _start function
/*fn main() {
    //reason for this error is that the println macro is part of the standard library, which we no longer include
    //println!("Hello, world!");
    //we need to define the panic handler our selves
} */
#[unsafe(no_mangle)] // we disable name mangling to ensure that the Rust compiler really outputs a function with the name _start
/* Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function a unique name. The attribute is required because we need to tell the name of the entry point function to the linker in the next step. */
pub extern "C" fn _start() -> ! {
    loop {}
}
/*Abort immediately → stop the program without cleaning up.

Unwind the stack → walk back through the function calls, cleaning up as it goes.

Stack unwinding is option 2:
Rust goes backwards through the call stack, dropping (cleaning up) variables one by one until it reaches the top or a catch_unwind handler. */
//stack unwinding diabled from cargo.toml removing by setting a panic strategy
