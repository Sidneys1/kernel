use crate::cmos;

pub struct SystemTime {
	pub year: u16,
	pub month: u16,
	pub day_of_week: u16,
	pub day: u16,
	pub hour: u16,
	pub minute: u16,
	pub second: u16,
	pub milliseconds: u16,
}

fn read_registers() -> (u8, u8, u8, u8, u8, u8, u8) {
	let second = cmos::read(0x00);
	let minute = cmos::read(0x02);
	let hour = cmos::read(0x04);
	let day_of_week = cmos::read(0x07);
	let day = cmos::read(0x07);
	let month = cmos::read(0x08);
	let year = cmos::read(0x09);

	(second, minute, hour, day_of_week, day, month, year)
}

pub fn get_system_time() -> SystemTime {
	use x86_64::instructions::interrupts::without_interrupts;
	without_interrupts(|| {
		let status_b = cmos::read(0x0b);
		let (mut second, mut minute, mut hour, mut day_of_week, mut day, mut month, mut year) = read_registers();
		if !(status_b & 0x04 == 0x04) {
			day = cmos::bcd_to_binary(day);
			month = cmos::bcd_to_binary(month);
			year = cmos::bcd_to_binary(year);
			day_of_week = cmos::bcd_to_binary(day_of_week);
			hour = cmos::bcd_to_binary(hour);
			minute = cmos::bcd_to_binary(minute);
			second = cmos::bcd_to_binary(second);
		};

		let mut year_long: u16 = year.into();
		year_long += 2000;

		SystemTime {
			year: year_long,
			month: month.into(),
			day_of_week: day_of_week.into(),
			day: day.into(),
			hour: hour.into(),
			minute: minute.into(),
			second: second.into(),
			milliseconds: 0,
		}
	})
}
