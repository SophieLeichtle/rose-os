#![no_std]
#![no_main] 

#![feature(custom_test_frameworks)]
#![test_runner(rose_os::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rose_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    rose_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("No Crashes!");
    rose_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rose_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rose_os::test_panic_handler(info)
}