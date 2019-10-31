pub fn bcd_to_binary(bcd: u8) -> u8 {
	return (bcd & 0x0f) + ((bcd >> 4) * 10);
}

pub fn read(register: u8) -> u8 {
	use x86_64::instructions::port::Port;
	let mut port_70 = Port::new(0x70);
	let mut port_71 = Port::new(0x71);
	unsafe {
		port_70.write(register);
		port_71.read()
	}
}
