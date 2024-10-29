#![allow(dead_code)]

pub fn calc_population(range: usize) -> Vec<usize> {
	let mut accum: Vec<usize> = Vec::new();
	accum.push(range);
	let mut cnt = (range - 1) * 2;

	for i in 1..range {
		accum.push(cnt);
		cnt -= 2;
	}

	accum
}
