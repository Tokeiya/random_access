mod calc_population;
mod generate_result;
mod hardware_random;
mod range_sizes;
mod result_writer;
mod sample_generator;
mod xorshift_64;

use crate::hardware_random::HardRnd;
use crate::hardware_random::RandomGenerator;
use calc_population::calc_population;
use generate_result::GenerateResult;
use rand_core::RngCore;
use rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use range_sizes::RangeSizes;
use sample_generator::*;
use std::fs::File;

use rayon::prelude::*;

fn build_path(range: RangeSizes, iteration: usize, file_name: &str) -> String {
	const arch: &str = if cfg!(target_arch = "x86_64") {
		"x86_64"
	} else if cfg!(target_arch = "aarch64") {
		"aarch64"
	} else {
		"unknown"
	};

	format!("./artifacts/{arch}_{range}_{iteration}_{file_name}")
}

fn main() {
	let size = vec![
		(RangeSizes::Size16, 16384),
		(RangeSizes::Size512, 16),
		(RangeSizes::Size1_024, 4),
		(RangeSizes::Size2_048, 1),
		(RangeSizes::Size4_096, 1),
	];

	size.par_iter().for_each(|(rng, ite)| {
		let mut theoretical = TheoreticalGenerator;
		let mut xorshift64 = XorShift64Generator::new(11_350_246_256_191_930_912);
		let mut hardware = HardwareGenerator::new();
		let mut xorshiro = XorShiroSampleGenerator::new(1_977_334_479_820_102_579);

		println!("range:{} size:{}", rng, ite);

		println!("theoretical");
		let result = theoretical.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "theoretical")).unwrap();
		result_writer::write_result(&mut writer, &result).unwrap();

		println!("xorshift64");
		let result = xorshift64.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "xorshift64")).unwrap();

		println!("xorshiro64");
		let result = xorshiro.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "xorshiro64")).unwrap();

		println!("hardware");
		let result = hardware.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "hardware")).unwrap();
		result_writer::write_result(&mut writer, &result).unwrap();
	})
}
