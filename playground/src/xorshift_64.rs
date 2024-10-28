pub struct Xorshift64(u64);

impl From<u64> for Xorshift64 {
	fn from(seed: u64) -> Self {
		Xorshift64(seed)
	}
}
impl Xorshift64 {
	pub fn next(&mut self) -> u64 {
		let mut x = self.0;
		x ^= x << 7;
		x ^= x >> 9;
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
			5408,
			688226,
			88609850,
			11275511870,
			1451700130917,
			184744262019830,
			23786025580950814,
			3026747159226705955,
			3062764496190468895,
			7648275456049016417,
			8649506351153624206,
			8983391669610725687,
			2957303707554776602,
			12422734192612848991,
			11465237076096684882,
			1246977212838145615,
			13116343009787186923,
			13047496037391138134,
			4327482731044219133,
			4266198335018721238,
		];

		let mut rnd = Xorshift64::from(42);

		for exp in EXPECTED.iter() {
			assert_eq!(*exp, rnd.next());
		}
	}
}
