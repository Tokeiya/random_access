mod calc_population;
mod generate_result;
mod hardware_random;
mod range_sizes;
mod result_writer;
mod sample_generator;
mod xorshift_64;

use crate::hardware_random::RandomGenerator;
use calc_population::calc_population;
use generate_result::GenerateResult;
use rand_core::RngCore;
use rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
use range_sizes::RangeSizes;
use sample_generator::*;
use std::fs::File;

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
	let mut generator = TheoreticalGenerator;
	let result = generator.generate(RangeSizes::Size256, 1);
	let mut writer =
		std::fs::File::create(build_path(RangeSizes::Size256, 1, "theoretical")).unwrap();

	result_writer::write_result(&mut writer, &result).unwrap();
}
