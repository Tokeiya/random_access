mod recent;
mod xorshift_64;

use std::fs::File;
use std::io::{Result as IoResult, Write};

fn write_result(result: &[usize], mut write: impl Write) -> IoResult<()> {
	writeln!(write, "diff\tcount")?;

	for (idx, cnt) in result.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}")?;
	}
	Ok(())
}
fn main() -> IoResult<()> {
	const MASK: u64 = 0x07_FF;

	let mut rnd = xorshift_64::Xorshift64::from(42);
	let mut accum = [0usize; 2048];

	let file = File::create("./artifacts/xorshift_result.tsv")?;

	let mut recent = rnd.next() & MASK;
	for _ in 0usize..0x80_00_00 {
		let current = rnd.next() & MASK;
		let idx = recent.abs_diff(current) as usize;
		accum[idx] += 1;
		recent = current;
	}

	write_result(&accum, file)?;

	Ok(())
}
