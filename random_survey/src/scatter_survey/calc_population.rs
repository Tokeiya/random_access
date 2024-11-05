#![allow(dead_code)]
use super::generate_result::GenerateResult;
use super::range_sizes::RangeSizes;

pub fn calc_population(range: RangeSizes, iteration: usize) -> Vec<GenerateResult> {
	let mut accum: Vec<GenerateResult> = Vec::new();
	accum.push(GenerateResult::new(
		iteration * range.to_usize(),
		range.to_usize() * iteration,
	));
	let mut cnt = (range.to_usize() - 1) * 2;

	for _ in 1..range.to_usize() {
		accum.push(GenerateResult::new(
			iteration * range.to_usize(),
			cnt * iteration,
		));
		cnt -= 2;
	}

	accum
}
