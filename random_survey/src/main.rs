#![feature(test)]

mod random_bench;
mod scatter_survey;

fn main() {
	scatter_survey::gen_sample();
}
