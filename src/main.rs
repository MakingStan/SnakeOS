#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
mod vga_buffer;
pub mod interrupts;
pub mod gdt;
mod snake_handler;

use core::panic::PanicInfo;
use crate::vga_buffer::Color;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    snake_handler::game_loop();

    hlt_loop()
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt_loop();
}

pub fn hlt_loop() -> ! {
    println!("got to htl loop");
    loop {
        x86_64::instructions::hlt();
    }
}