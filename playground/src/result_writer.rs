use std::fs::File;
use std::io::{Result as IoResult, Write};

pub fn write_result(mut write: impl Write, data: &[usize]) -> IoResult<()> {
	writeln!(write, "diff\tcount")?;

	for (diff, cnt) in data.iter().enumerate() {
		writeln!(write, "{diff}\t{cnt}")?;
	}

	Ok(())
}
