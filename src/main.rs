#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod macros;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("test");
    print!("rtrr");
    println!("test2");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
