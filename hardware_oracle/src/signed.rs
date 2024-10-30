pub trait Signed {
	fn is_negative(&self) -> bool;
}

impl Signed for i64 {
	fn is_negative(&self) -> bool {
		i64::is_negative(*self)
	}
}

impl Signed for u64 {
	fn is_negative(&self) -> bool {
		false
	}
}

impl Signed for i32 {
	fn is_negative(&self) -> bool {
		i32::is_negative(*self)
	}
}

impl Signed for u32 {
	fn is_negative(&self) -> bool {
		false
	}
}

impl Signed for i16 {
	fn is_negative(&self) -> bool {
		i16::is_negative(*self)
	}
}

impl Signed for u16 {
	fn is_negative(&self) -> bool {
		false
	}
}

impl Signed for i8 {
	fn is_negative(&self) -> bool {
		i8::is_negative(*self)
	}
}

impl Signed for u8 {
	fn is_negative(&self) -> bool {
		false
	}
}

impl Signed for isize {
	fn is_negative(&self) -> bool {
		isize::is_negative(*self)
	}
}

impl Signed for usize {
	fn is_negative(&self) -> bool {
		false
	}
}
