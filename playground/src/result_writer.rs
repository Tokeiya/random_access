use super::generate_result::GenerateResult;
use std::fs::File;
use std::io::{Result as IoResult, Write};

static ARCH: &str = if cfg!(target_arch = "x86_64") {
	"x86_64"
} else if cfg!(target_arch = "aarch64") {
	"aarch64"
} else if cfg!(target_arch = "arm") {
	"arm"
} else {
	"unknown"
};

pub fn write_result(mut write: impl Write, method: &str, data: &[GenerateResult]) -> IoResult<()> {
	writeln!(write, "arch\tmethod\tsize\tindex\tcount\tdiff")?;

	for (idx, result) in data.iter().enumerate() {
		writeln!(
			write,
			"{ARCH}\t{method}\t{}\t{idx}\t{}\t{}",
			data.len(),
			result.count(),
			result.diff()
		)?;
	}

	Ok(())
}
