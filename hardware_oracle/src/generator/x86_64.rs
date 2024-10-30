#![cfg(target_arch = "x86_64")]
use std::arch::x86_64::_rdrand64_step;

pub fn oracle() -> u64 {
	let mut buf = 0u64;

	if unsafe { _rdrand64_step(&mut buf) } == 0 {
		panic!("rdrand failed")
	} else {
		buf
	}
}
