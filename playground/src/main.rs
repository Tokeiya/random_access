mod calc_population;
mod hardware_random;
mod result_writer;

use crate::hardware_random::RandomGenerator;
use calc_population::calc_population;
use std::fs::File;

const RANGE: usize = 2048;
const SIZE: usize = 2048 * 2048usize;
const MASK: u64 = 0x07_FF;
fn hard_rnd() -> [usize; RANGE] {
	println!("enter hard_rnd");
	let mut accum: [usize; RANGE] = [0; RANGE];
	let mut rnd = hardware_random::HardRnd::default();

	let mut recent = rnd.get_random_u64() & MASK;

	for i in 0..SIZE {
		if (i & 0x0F_FF == 0) {
			println!("{i}/{SIZE}")
		}

		let current = rnd.get_random_u64() & MASK;
		accum[recent.abs_diff(current) as usize] += 1;
		recent = current;
	}

	accum
}
use std::env;
use std::path::PathBuf;

fn build_path(file_name: &str) -> String {
	const arch: &str = if cfg!(target_arch = "x86_64") {
		"x86_64"
	} else if cfg!(target_arch = "aarch64") {
		"aarch64"
	} else {
		"unknown"
	};

	format!("./artifacts/{arch}_{file_name}")
}

fn main() {
	let file = File::create("./artifacts/population.tsv").unwrap();
	result_writer::write_result(file, &calc_population(2048)).unwrap();

	let file = File::create(build_path("hard_rand.tsv")).unwrap();
	let accum = hard_rnd();
	result_writer::write_result(file, &hard_rnd()).unwrap();
}
