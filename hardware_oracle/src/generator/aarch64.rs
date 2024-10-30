#![cfg(target_arch = "aarch64")]

use std::fs::File;
use std::io::Read;

pub fn oracle() -> u64 {
	let mut file = File::open("/dev/urandom").unwrap();
	let mut buf = [0u8; 8];
	file.read(&mut buf).unwrap();

	if cfg!(target_endian = "little") {
		u64::from_le_bytes(buf)
	} else {
		u64::from_be_bytes(buf)
	}
}
