// ignore-macos
// ignore-windows

#![warn(clippy::empty_loop)]
#![feature(lang_items, link_args, start, libc)]
#![link_args = "-nostartfiles"]
#![no_std]

use core::panic::PanicInfo;

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    // This loop should not cause a warning.
    let dummy = 0u8;
    loop {
        unsafe { core::ptr::read_volatile(&dummy) };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // But this loop will cause a warning.
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
