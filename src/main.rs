#![no_std]
#![feature(no_core)]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Yo");
    println!("Guys");
    for _ in 0..23 {
        println!("{}", 30.0/7.0);
    }
    loop {}
}
