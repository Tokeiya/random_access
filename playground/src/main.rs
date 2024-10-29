mod calc_population;
mod hardware_random;
mod result_writer;

use crate::hardware_random::RandomGenerator;
use calc_population::calc_population;

fn main() {
	let mut foo = hardware_random::HardRnd::default();

	println!("{}", foo.get_random_u64())
}
