mod calc_population;
mod hardware_random;
mod result_writer;
mod xorshift_64;

use crate::hardware_random::RandomGenerator;
use calc_population::calc_population;
use std::fs::File;

const RANGE: usize = 2048;
const MULTIPLY: usize = 1;
const SIZE: usize = RANGE * RANGE * MULTIPLY;
const MASK: u64 = (RANGE - 1) as u64;

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

fn xorshift_rnd() -> [usize; RANGE] {
	let mut accum: [usize; RANGE] = [0; RANGE];
	let mut rnd = xorshift_64::Xorshift64::from(16_733_782_310_569_982_181);

	let mut recent = rnd.next() & MASK;

	for i in 0..SIZE {
		if (i & 0x0F_FF == 0) {
			println!("{i}/{SIZE}")
		}

		let current = rnd.next() & MASK;
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

	format!("./artifacts/{arch}_{RANGE}_{file_name}")
}

fn main() {
	let current_dir = std::env::current_dir().unwrap();
	println!("Current directory: {:?}", current_dir);

	let file = File::create(build_path("xorshift.tsv")).unwrap();
	result_writer::write_result(file, &xorshift_rnd()).unwrap();

	let file = File::create(build_path("hard_rnd.tsv")).unwrap();
	result_writer::write_result(file, &hard_rnd()).unwrap();

	let file = File::create(build_path("calc_population.tsv")).unwrap();
	result_writer::write_result(file, &calc_population::calc_population(RANGE, MULTIPLY)).unwrap();
	println!("done");
}
