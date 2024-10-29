pub trait RandomGenerator: Default {
	fn get_random_u64(&mut self) -> u64;
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
	use crate::hardware_random::RandomGenerator;
	use std::fs::File;
	use std::io::{BufReader, Read};

	pub struct HardwareRandom(BufReader<File>);

	impl Default for HardwareRandom {
		fn default() -> Self {
			HardwareRandom(BufReader::new(File::open("/dev/hwrng").unwrap()))
		}
	}
	impl RandomGenerator for HardwareRandom {
		fn get_random_u64(&mut self) -> u64 {
			let mut dta: [u8; 8] = [0; 8];
			self.0.read_exact(&mut dta).unwrap();
			u64::from_le_bytes(dta)
		}
	}
}

#[cfg(target_arch = "x86_64")]
mod x86_64 {
	use super::RandomGenerator;
	use std::arch::x86_64::_rdrand64_step;

	pub struct HardwareRandom;

	impl Default for HardwareRandom {
		fn default() -> Self {
			HardwareRandom
		}
	}
	impl RandomGenerator for HardwareRandom {
		fn get_random_u64(&mut self) -> u64 {
			let mut dta: u64 = 0;
			_ = unsafe { _rdrand64_step(&mut dta) };

			dta
		}
	}
}

#[cfg(target_arch = "aarch64")]
pub use aarch64::HardwareRandom as HardRnd;

#[cfg(target_arch = "x86_64")]
pub use x86_64::HardwareRandom as HardRnd;
