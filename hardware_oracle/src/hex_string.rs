pub trait HexString {
	fn to_hex_string(&self) -> String;
}

impl HexString for u64 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for i64 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for u32 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for i32 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for u16 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for i16 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for u8 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for i8 {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for usize {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}

impl HexString for isize {
	fn to_hex_string(&self) -> String {
		format!("{:X}", self)
	}
}
