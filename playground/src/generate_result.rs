pub struct GenerateResult {
	count: usize,
	diff: usize,
}

impl GenerateResult {
	pub fn new(count: usize, diff: usize) -> Self {
		Self { count, diff }
	}

	pub fn count(&self) -> usize {
		self.count
	}

	pub fn diff(&self) -> usize {
		self.diff
	}

	pub fn inclement_count(&mut self) {
		self.count += 1;
	}

	pub fn inclement_diff(&mut self) {
		self.diff += 1;
	}
}
