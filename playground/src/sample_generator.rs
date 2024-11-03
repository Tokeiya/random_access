use super::calc_population::calc_population;
use super::generate_result::GenerateResult;
use super::range_sizes::RangeSizes;
use super::xorshift_64::Xorshift64;
use crate::hardware_random::{HardRnd, RandomGenerator};
use rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

pub trait SampleGenerator {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult>;
}

pub struct TheoreticalGenerator;

impl SampleGenerator for TheoreticalGenerator {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		calc_population(range, iteration)
	}
}

pub struct XorShift64Generator(Xorshift64);

impl XorShift64Generator {
	pub fn new(seed: u64) -> Self {
		XorShift64Generator(Xorshift64::from(seed))
	}
}

impl SampleGenerator for XorShift64Generator {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		let mut data = Vec::with_capacity(iteration);
		let ite = range.to_usize() * range.to_usize() * iteration;

		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mask = range.to_usize() - 1;
		let mut recent = self.0.next() as usize & mask;

		for _ in 0..ite {
			let idx = self.0.next() as usize & mask;
			let diff = recent.abs_diff(idx);

			data[idx].inclement_count();
			data[recent].inclement_diff();
		}

		data
	}
}

pub struct XorShiroSampleGenerator(Xoshiro256StarStar);

impl XorShiroSampleGenerator {
	pub fn new(seed: u64) -> Self {
		XorShiroSampleGenerator(Xoshiro256StarStar::seed_from_u64(seed))
	}
}

impl SampleGenerator for XorShiroSampleGenerator {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		let mut data = Vec::new();
		let ite = range.to_usize() * range.to_usize() * iteration;
		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mask = range.to_usize() - 1;
		let mut recent = self.0.next_u64() as usize & mask;

		for _ in 0..ite {
			let idx = self.0.next_u64() as usize & mask;
			let diff = recent.abs_diff(idx);

			data[idx].inclement_count();
			data[diff].inclement_diff();
			recent = idx;
		}

		data
	}
}

pub struct HardwareGenerator<T>(T);

impl HardwareGenerator<HardRnd> {
	pub fn new() -> Self {
		HardwareGenerator(HardRnd::default())
	}
}

impl<T: RandomGenerator> SampleGenerator for HardwareGenerator<T> {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		let mut data = Vec::new();
		let ite = range.to_usize() * range.to_usize() * iteration;
		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mask = range.to_usize() - 1;
		let mut recent = self.0.get_random_u64() as usize & mask;

		for i in 0..ite {
			if i & 0xFFFF == 0 {
				println!("{i}/{ite}");
			}

			let idx = self.0.get_random_u64() as usize & mask;
			let diff = recent.abs_diff(idx);

			data[idx].inclement_count();
			data[diff].inclement_diff();
			recent = idx;
		}

		data
	}
}
