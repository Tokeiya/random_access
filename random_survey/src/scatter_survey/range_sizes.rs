use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy)]
pub enum RangeSizes {
	Size2,
	Size4,
	Size8,
	Size16,
	Size32,
	Size64,
	Size128,
	Size256,
	Size512,
	Size1_024,
	Size2_048,
	Size4_096,
	Size8_192,
	Size16_384,
	Size32_768,
	Size65_536,
	Size131_072,
	Size262_144,
	Size524_288,
	Size1_048_576,
	Size2_097_152,
	Size4_194_304,
	Size8_388_608,
	Size16_777_216,
	Size33_554_432,
	Size67_108_864,
	Size134_217_728,
	Size268_435_456,
	Size536_870_912,
	Size1_073_741_824,
	Size2_147_483_648,
	Size4_294_967_296,
}
impl RangeSizes {
	pub fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			RangeSizes::Size2 => "2",
			RangeSizes::Size4 => "4",
			RangeSizes::Size8 => "8",
			RangeSizes::Size16 => "16",
			RangeSizes::Size32 => "32",
			RangeSizes::Size64 => "64",
			RangeSizes::Size128 => "128",
			RangeSizes::Size256 => "256",
			RangeSizes::Size512 => "512",
			RangeSizes::Size1_024 => "1_024",
			RangeSizes::Size2_048 => "2_048",
			RangeSizes::Size4_096 => "4_096",
			RangeSizes::Size8_192 => "8_192",
			RangeSizes::Size16_384 => "16_384",
			RangeSizes::Size32_768 => "32_768",
			RangeSizes::Size65_536 => "65_536",
			RangeSizes::Size131_072 => "131_072",
			RangeSizes::Size262_144 => "262_144",
			RangeSizes::Size524_288 => "524_288",
			RangeSizes::Size1_048_576 => "1_048_576",
			RangeSizes::Size2_097_152 => "2_097_152",
			RangeSizes::Size4_194_304 => "4_194_304",
			RangeSizes::Size8_388_608 => "8_388_608",
			RangeSizes::Size16_777_216 => "16_777_216",
			RangeSizes::Size33_554_432 => "33_554_432",
			RangeSizes::Size67_108_864 => "67_108_864",
			RangeSizes::Size134_217_728 => "134_217_728",
			RangeSizes::Size268_435_456 => "268_435_456",
			RangeSizes::Size536_870_912 => "536_870_912",
			RangeSizes::Size1_073_741_824 => "1_073_741_824",
			RangeSizes::Size2_147_483_648 => "2_147_483_648",
			RangeSizes::Size4_294_967_296 => "4_294_967_296",
		};

		f.write_str(str)
	}

	pub fn to_usize(&self) -> usize {
		match self {
			RangeSizes::Size2 => 2,
			RangeSizes::Size4 => 4,
			RangeSizes::Size8 => 8,
			RangeSizes::Size16 => 16,
			RangeSizes::Size32 => 32,
			RangeSizes::Size64 => 64,
			RangeSizes::Size128 => 128,
			RangeSizes::Size256 => 256,
			RangeSizes::Size512 => 512,
			RangeSizes::Size1_024 => 1_024,
			RangeSizes::Size2_048 => 2_048,
			RangeSizes::Size4_096 => 4_096,
			RangeSizes::Size8_192 => 8_192,
			RangeSizes::Size16_384 => 16_384,
			RangeSizes::Size32_768 => 32_768,
			RangeSizes::Size65_536 => 65_536,
			RangeSizes::Size131_072 => 131_072,
			RangeSizes::Size262_144 => 262_144,
			RangeSizes::Size524_288 => 524_288,
			RangeSizes::Size1_048_576 => 1_048_576,
			RangeSizes::Size2_097_152 => 2_097_152,
			RangeSizes::Size4_194_304 => 4_194_304,
			RangeSizes::Size8_388_608 => 8_388_608,
			RangeSizes::Size16_777_216 => 16_777_216,
			RangeSizes::Size33_554_432 => 33_554_432,
			RangeSizes::Size67_108_864 => 67_108_864,
			RangeSizes::Size134_217_728 => 134_217_728,
			RangeSizes::Size268_435_456 => 268_435_456,
			RangeSizes::Size536_870_912 => 536_870_912,
			RangeSizes::Size1_073_741_824 => 1_073_741_824,
			RangeSizes::Size2_147_483_648 => 2_147_483_648,
			RangeSizes::Size4_294_967_296 => 4_294_967_296,
		}
	}
}
impl Debug for RangeSizes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}

impl Display for RangeSizes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.format(f)
	}
}
