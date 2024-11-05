#![feature(test)]

fn main() {
	println!("Hello, world!");
}

#[cfg(test)]
mod bench {
	extern crate test;
	use test::Bencher;
	
	
	#[bench]
	
}