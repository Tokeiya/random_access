#![allow(dead_code)]

use std::fs::File;

use crate::scatter_survey::range_sizes::RangeSizes;
use crate::scatter_survey::result_writer;
use crate::scatter_survey::sample_generator::{
	HardwareGenerator, SampleGenerator, TheoreticalGenerator, XorShift64Generator,
	Xoshiro256StarStarGenerator,
};
use rayon::prelude::*;

fn build_path(range: RangeSizes, iteration: usize, file_name: &str) -> String {
	const ARCH: &str = if cfg!(target_arch = "x86_64") {
		"x86_64"
	} else if cfg!(target_arch = "aarch64") {
		"aarch64"
	} else {
		"unknown"
	};

	format!("./artifacts/{ARCH}_{range}_{iteration}_{file_name}")
}

fn gen_sample() {
	let size = vec![
		(RangeSizes::Size4, 1048576),
		(RangeSizes::Size8, 524288),
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
		let mut xorshiro = Xoshiro256StarStarGenerator::new(1_977_334_479_820_102_579);

		println!("range:{} size:{}", rng, ite);

		println!("theoretical");
		let result = theoretical.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "theoretical.tsv")).unwrap();
		result_writer::write_result(&mut writer, "theoretical", &result).unwrap();

		println!("xorshift64");
		let result = xorshift64.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "xorshift64.tsv")).unwrap();
		result_writer::write_result(&mut writer, "xorshift64", &result).unwrap();

		println!("xorshiro64");
		let result = xorshiro.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "xorshiro64.tsv")).unwrap();
		result_writer::write_result(&mut writer, "xoshiro256**", &result).unwrap();

		println!("hardware");
		let result = hardware.generate(*rng, *ite);
		let mut writer = File::create(build_path(*rng, *ite, "hardware.tsv")).unwrap();
		result_writer::write_result(&mut writer, "hardware_rnd", &result).unwrap();
	})
}
