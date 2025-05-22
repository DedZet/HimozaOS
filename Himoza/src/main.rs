#![no_std]
#![no_main]

mod vga_buffer;

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#![reexport_test_harness_main = "test_main"]

use core::{panic::PanicInfo};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* fn print(text: &[u8]) {
//     let vga_buffer:*mut u8 = 0xb8000 as *mut u8;
//     for (i, &byte) in text.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xe; //b - blue; a - green; c - red;
//         }}} */

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello, world!\n");
    
    loop {}
}


