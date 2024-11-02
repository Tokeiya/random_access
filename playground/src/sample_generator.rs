use super::calc_population::calc_population;
use super::generate_result::GenerateResult;
use super::range_sizes::RangeSizes;
use super::xorshift_64::Xorshift64;
use crate::hardware_random::{HardRnd, RandomGenerator};
use rand_core::RngCore;
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
		let ite = range.to_usize() * iteration;

		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mut recent = self.0.next();

		for _ in 0..ite {
			let idx = self.0.next();
			let diff = recent.abs_diff(idx);

			data[idx as usize].inclement_count();
			data[recent as usize].inclement_diff();
		}

		data
	}
}

pub struct XorShiroSampleGenerator(Xoshiro256StarStar);

impl SampleGenerator for XorShiroSampleGenerator {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		let mut data = Vec::new();
		let ite = range.to_usize() * iteration;
		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mut recent = self.0.next_u64();

		for _ in 0..ite {
			let idx = self.0.next_u64();
			let diff = recent.abs_diff(idx);

			data[idx as usize].inclement_count();
			data[recent as usize].inclement_diff();
			recent = idx;
		}

		data
	}
}

pub struct HardwareGenerator<T>(T);

impl<T: RandomGenerator> SampleGenerator for HardwareGenerator<T> {
	fn generate(&mut self, range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
		let mut data = Vec::new();
		let ite = range.to_usize() * iteration;
		for _ in 0..range.to_usize() {
			data.push(GenerateResult::new(0, 0));
		}

		let mut recent = self.0.get_random_u64();

		for _ in 0..ite {
			let idx = self.0.get_random_u64();
			let diff = recent.abs_diff(idx);

			data[idx as usize].inclement_count();
			data[recent as usize].inclement_diff();
			recent = idx;
		}

		data
	}
}
