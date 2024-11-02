use super::generate_result::GenerateResult;
use std::fs::File;
use std::io::{Result as IoResult, Write};

pub fn write_result(mut write: impl Write, data: &[GenerateResult]) -> IoResult<()> {
	writeln!(write, "index\tcount\tdiff")?;

	for (idx, result) in data.iter().enumerate() {
		writeln!(write, "{idx}\t{}\t{}", result.count(), result.diff())?;
	}

	Ok(())
}
