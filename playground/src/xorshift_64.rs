pub struct Xorshift64(u64);

impl From<u64> for Xorshift64 {
	fn from(seed: u64) -> Self {
		Xorshift64(seed)
	}
}
impl Xorshift64 {
	pub fn next(&mut self) -> u64 {
		let mut x = self.0;
		x ^= x << 13;
		x ^= x >> 7;
		x ^= x << 17;
		self.0 = x;
		x
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn next() {
		const EXPECTED: [u64; 20] = [
			1870430064996970113,
			13435956771586562844,
			16582051418305547178,
			4363812670234057661,
			6859429352946554954,
			258867383544808058,
			6329142421382521854,
			11983960629208147129,
			12584166297092064560,
			5023263820646707770,
			18004544384661803870,
			4855001950758325880,
			2066141214554873572,
			17934390888769112921,
			13269649994388477583,
			4275150340138516834,
			3614940304489474144,
			9303812959960226992,
			7802347781056087305,
			10142083201847158979,
		];

		let mut rnd = Xorshift64::from(609302558241040659);

		for exp in EXPECTED.iter() {
			assert_eq!(*exp, rnd.next());
		}
	}
}
