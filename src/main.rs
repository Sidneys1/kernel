#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::println;

entry_point!(kernel_main);



fn kernel_main(boot_info: &'static BootInfo) -> ! {
	use kernel::allocator;
	use kernel::memory;
	use x86_64::VirtAddr;
	kernel::init();

	println!("Initializing Kernel...");

	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
	let mut mapper = unsafe { memory::init(phys_mem_offset) };
	let mut frame_allocator =
		unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
	allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

	#[cfg(test)]
	test_main();

	kernel::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	kernel::test_panic_handler(info)
}
