#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Add test runner
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Since custom test frameworks line above generates its own main which is ignored,
// We need to redirect it to the test_runner function in this file.
// Set name of test entry function

#![reexport_test_harness_main = "test_main"]

// Imports
mod vga_buffer;
mod serial;
use core::panic::PanicInfo;


// This function is called on panic.
#[cfg(not(test))] // Only compile this when we aren't testing.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    
    loop {}
}

// Entry point
// this function is the entry point rather than main(), 
// since the linker looks for a function named `_start` by default
#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    print!("Hello, ");
    println!("world{}", "!");

    #[cfg(test)] // Only include when running tests
    test_main();

    // panic!("Something went wrong.");
    
    loop {}
}


pub trait Testable {
    fn run(&self) -> ();
}


impl<T> Testable for T where T: Fn() {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


#[cfg(test)] // Only include this for tests
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
