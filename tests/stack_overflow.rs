#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use kernel::serial_print;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	serial_print!("stack_overflow... ");

	kernel::gdt::init();
	init_test_idt();

	// trigger a stack overflow
	stack_overflow();

	panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
	stack_overflow(); // for each recursion, the return address is pushed
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	kernel::test_panic_handler(info)
}

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
	static ref TEST_IDT: InterruptDescriptorTable = {
		let mut idt = InterruptDescriptorTable::new();
		unsafe {
			idt.double_fault
				.set_handler_fn(test_double_fault_handler)
				.set_stack_index(kernel::gdt::DOUBLE_FAULT_IST_INDEX);
		}

		idt
	};
}

pub fn init_test_idt() {
	TEST_IDT.load();
}

use kernel::{exit_qemu, serial_println, QemuExitCode};
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_double_fault_handler(
	_stack_frame: &mut InterruptStackFrame,
	_error_code: u64,
) {
	serial_println!("[ok]");
	exit_qemu(QemuExitCode::Success);
	loop {}
}
