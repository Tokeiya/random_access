mod recent;
mod xorshift_64;

use std::fs::File;
use std::io::{Result as IoResult, Write};

const MASK: u64 = 0x07_FF;

fn write_result(result: &[usize], mut write: impl Write) -> IoResult<()> {
	writeln!(write, "diff\tcount")?;

	for (idx, cnt) in result.iter().enumerate() {
		writeln!(write, "{idx}\t{cnt}")?;
	}
	Ok(())
}
fn main() {
	xorshift::Xo
}
