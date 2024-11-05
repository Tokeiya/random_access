#[cfg(test)]
mod bench {
	extern crate test;
	use rand_core::{RngCore, SeedableRng};
	use rand_xoshiro::{SplitMix64, Xoroshiro64Star, Xoroshiro64StarStar};
	use test::Bencher;

	const ITERATION: usize = 500_000;
	#[bench]
	fn bench_rand_xoshiro64_star_star(b: &mut Bencher) {
		let mut accum: u64 = 0;

		b.iter(|| {
			let mut rng = Xoroshiro64StarStar::seed_from_u64(1_977_334_479_820_102_579);

			for _ in 0..ITERATION {
				accum ^= rng.next_u64();
			}
		});

		println!("{accum}")
	}

	#[bench]
	fn bench_rand_xoshiro64_star(b: &mut Bencher) {
		let mut accum: u64 = 0;

		b.iter(|| {
			let mut rng = Xoroshiro64Star::seed_from_u64(1_977_334_479_820_102_579);

			for _ in 0..ITERATION {
				accum ^= rng.next_u64();
			}
		});

		println!("{accum}")
	}

	#[bench]
	fn bench_rand_splitmix64(b: &mut Bencher) {
		let mut accum: u64 = 0;

		b.iter(|| {
			let mut rng = SplitMix64::seed_from_u64(1_977_334_479_820_102_579);

			for _ in 0..ITERATION {
				accum ^= rng.next_u64();
			}
		});

		println!("{accum}")
	}

	#[bench]
	fn sequence(b: &mut Bencher) {
		let mut accum = 0u64;

		b.iter(|| {
			for i in 0..(ITERATION as u64) {
				accum ^= i;
			}
		});
		println!("{accum}");
	}
}
