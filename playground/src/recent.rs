#![allow(dead_code)]

use rand::Rng;
use rand_core::{RngCore, SeedableRng};
use std::fs::File;
use std::io::{BufWriter, Write};

pub(crate) fn foo() {
	const MASK: usize = 0x07ff;
	const SHIFT_MASK: usize = 0x0F;

	let mut rnd = sfmt::SFMT::seed_from_u64(114514);
	let arr: [usize; 2048] = std::array::from_fn(|_| rnd.gen_range(0xFF_FF..usize::MAX));
	{
		let mut file = File::create("./artifacts/shift_arr1.tsv").unwrap();

		writeln!(file, "idx\tval").unwrap();
		for (idx, val) in arr.iter().enumerate() {
			writeln!(file, "{idx}\t{val}").unwrap();
		}
	}

	let file = File::create("./artifacts/shift_result1.tsv").unwrap();
	let mut write = BufWriter::new(file);

	writeln!(write, "diff\tcount").unwrap();
	let mut accum: usize = rnd.next_u64() as usize;
	let mut recent = accum & MASK;

	let mut accum_tbl = [0usize; 2048];

	for i in 0usize..0x4000000 {
		accum = accum.wrapping_add(arr[recent]);

		let tmp = if recent + 1 == 2048 { 0 } else { recent + 1 };

		accum = accum >> (accum & SHIFT_MASK);
		accum = accum.wrapping_add(arr[tmp]);

		let tmp = if recent.wrapping_sub(1) == usize::MAX {
			0
		} else {
			recent.wrapping_sub(1)
		};

		accum = accum.wrapping_add(arr[tmp]);
		accum = accum << (accum & SHIFT_MASK);
		accum = accum.wrapping_add(i);

		let current = accum & MASK;
		let diff = usize::abs_diff(recent, current);
		accum_tbl[diff] += 1;
		recent = current;
	}

	for (idx, cnt) in accum_tbl.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}").unwrap();
	}
}

pub(crate) fn add_accum() {
	const MASK: usize = 0x07ff;

	let mut rnd = sfmt::SFMT::seed_from_u64(114514);
	let arr: [usize; 2048] = std::array::from_fn(|_| rnd.gen_range(0xFF_FF..usize::MAX));
	{
		let mut file = File::create("./artifacts/arr1.tsv").unwrap();

		writeln!(file, "idx\tval").unwrap();
		for (idx, val) in arr.iter().enumerate() {
			writeln!(file, "{idx}\t{val}").unwrap();
		}
	}

	let file = File::create("./artifacts/add_result1.tsv").unwrap();
	let mut write = BufWriter::new(file);

	writeln!(write, "diff\tcount").unwrap();
	let mut accum: usize = rnd.next_u64() as usize;
	let mut recent = accum & MASK;

	let mut accum_tbl = [0usize; 2048];

	for i in 0usize..0x4000000 {
		accum = accum.wrapping_add(arr[recent]);
		accum = accum.wrapping_add(i);
		let current = accum & MASK;
		let diff = usize::abs_diff(recent, current);
		accum_tbl[diff] += 1;
		recent = current;
	}

	for (idx, cnt) in accum_tbl.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}").unwrap();
	}
}

pub(crate) fn rnd_diff() {
	let mut rnd = sfmt::SFMT::seed_from_u64(42);
	let file = File::create("./artifacts/rnd_result.tsv").unwrap();

	let mut accum = [0usize; 2048];

	for _ in 0..4194304usize {
		let idx = usize::abs_diff(rnd.gen_range(0..2048), rnd.gen_range(0..2048));
		accum[idx] += 1;
	}

	let mut write = BufWriter::new(file);
	writeln!(write, "diff\tcount").unwrap();
	for (idx, cnt) in accum.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}").unwrap();
	}
}

pub(super) fn calc_population() {
	let file = File::create("./artifacts/pop_result.tsv").unwrap();
	let mut write = BufWriter::new(file);

	let mut accum = [0usize; 2048];

	for a in 0usize..2048 {
		for b in 0usize..2048 {
			accum[usize::abs_diff(a, b)] += 1;
		}
	}
	writeln!(write, "diff\tcount").unwrap();
	for (idx, cnt) in accum.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}").unwrap();
	}
}
