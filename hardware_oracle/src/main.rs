mod generator;
mod hex_string;
mod signed;

use hex_string::HexString;
use signed::Signed;
pub fn thousand_separator<T: ToString + Signed>(value: &T) -> String {
	let mut buff = String::new();
	let scr = value.to_string();

	let mut cnt = 3 - (scr.len() % 3);
	let mut iter = scr.chars();

	if value.is_negative() {
		buff.push(iter.next().unwrap());
		cnt += 1;
	}

	for c in iter {
		if cnt == 0 {
			buff.push('_');
		}
		buff.push(c);
		cnt += 1;
		cnt %= 3
	}

	buff
}

pub fn hex_separator<T: HexString>(value: &T) -> String {
	let mut buff = String::from("0x");
	let scr = value.to_hex_string();

	let mut cnt = 0usize;

	if scr.len() % 2 == 1 {
		buff.push('0');
		cnt += 1;
	} else {
		cnt += 2;
	}

	for c in scr.chars() {
		if cnt == 0 {
			buff.push('_');
		}

		buff.push(c);
		cnt += 1;
		cnt %= 2;
	}

	buff
}

fn main() {
	let oracle = generator::oracle();

	println!(
		"u64:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = oracle as i64;
	println!(
		"i64:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = (oracle >> 32) as u32;
	println!(
		"u32:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = oracle as i32;
	println!(
		"i32:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = (oracle >> 16) as u16;
	println!(
		"u16:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = oracle as i16;
	println!(
		"i16:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = (oracle >> 8) as u8;
	println!(
		"u8:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	);

	let oracle = oracle as i8;
	println!(
		"i8:{}\t{}",
		thousand_separator(&oracle),
		hex_separator(&oracle)
	)
}
