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
    // ! -> not allowed to ever return.
    //mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function (instead of the unspecified Rust calling convention).
    loop {}
}
/*Abort immediately → stop the program without cleaning up.

Unwind the stack → walk back through the function calls, cleaning up as it goes.

Stack unwinding is option 2:
Rust goes backwards through the call stack, dropping (cleaning up) variables one by one until it reaches the top or a catch_unwind handler. */
//stack unwinding diabled from cargo.toml removing by setting a panic strategy

//RUST ASSUMES YOU ARE BUILDING FOR YOUR HOST SYSTEM HENCE LINKER ERROR
//But in OS development, you’re writing code that runs without an OS.
// linker still tries to pull in those runtime pieces, fails to find them, and throws errors.

//build for bare metal by rustup target add thumbv7em-none-eabihf which is an embedded library which you use to run your code
//pick thumbv7em-none-eabihf, you’re saying: “Build for an ARM Cortex‑M chip, with no OS, using hardware floating point.”

//By passing a --target argument we cross compile our executable for a bare metal target system. Since the target system has no operating system, the linker does not try to link the C runtime and our build succeeds without any linker errors.
