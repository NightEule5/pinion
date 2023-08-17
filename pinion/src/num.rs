// SPDX-License-Identifier: Apache-2.0

use std::ops::RangeBounds;

mod sealed {
	pub trait SealedNumExt { }
	impl SealedNumExt for i8 { }
	impl SealedNumExt for u8 { }
	impl SealedNumExt for i16 { }
	impl SealedNumExt for u16 { }
	impl SealedNumExt for i32 { }
	impl SealedNumExt for u32 { }
	impl SealedNumExt for i64 { }
	impl SealedNumExt for u64 { }
	//impl SealedNumExt for i128 { }
	//impl SealedNumExt for u128 { }
	impl SealedNumExt for isize { }
	impl SealedNumExt for usize { }
}

pub trait NumExt: PartialOrd<Self> + Sized + sealed::SealedNumExt {
	/// Optionally returns this number if it is not zero.
	fn non_zero(self) -> Option<Self>;
	/// Optionally returns this number if it is positive. Has the same effect as
	/// [`non_zero`][] for unsigned integers.
	///
	/// [`non_zero`]: NumExt::non_zero
	fn positive(self) -> Option<Self>;
	/// Optionally returns this number if it is greater than `other`.
	fn greater_than<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B>;
	/// Optionally returns this number if it is less than `other`.
	fn less_than<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B>;
	/// Optionally returns this number if it is greater than or equal to `other`.
	fn greater_than_or_equal<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B>;
	/// Optionally returns this number if it is less than or equal to `other`.
	fn less_than_or_equal<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B>;
	/// Optionally returns this number if it is within `range`'s bounds.
	fn in_range<R: RangeBounds<Self>>(self, range: R) -> Option<Self> {
		range.contains(&self).then_some(self)
	}
	/// Optionally returns this number if it is even.
	fn even(self) -> Option<Self> { self.is_even().then_some(self) }
	/// Optionally returns this number if it is odd.
	fn odd(self) -> Option<Self> { self.is_odd().then_some(self) }
	/// Returns `true` if this number is even.
	fn is_even(&self) -> bool;
	/// Returns `true` if this number is odd.
	fn is_odd(&self) -> bool { !self.is_even() }

	#[cfg(feature = "primes")]
	/// Optionally returns this number if it is a prime, using the [`primal`] crate.
	fn prime(self) -> Option<Self> {
		self.is_prime().then_some(self)
	}

	#[cfg(feature = "primes")]
	/// Returns `true` if this number is a prime, using the [`primal`] crate.
	fn is_prime(&self) -> bool;
}

pub trait SNumExt: NumExt {
	/// Optionally returns this number if it is negative.
	fn negative(self) -> Option<Self>;
}

macro_rules! nums {
    ($($ty:ident)+) => {
		$(
		impl NumExt for $ty {
			fn non_zero(self) -> Option<Self> {
				(self != 0).then_some(self)
			}

			fn positive(self) -> Option<Self> {
				(self > 0).then_some(self)
			}

			fn greater_than<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B> {
				(self > other).then_some(self)
			}

			fn less_than<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B> {
				(self < other).then_some(self)
			}

			fn greater_than_or_equal<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B> {
				(self >= other).then_some(self)
			}

			fn less_than_or_equal<B>(self, other: B) -> Option<Self> where Self: PartialOrd<B> {
				(self <= other).then_some(self)
			}

			fn is_even(&self) -> bool { self % 2 == 0 }

			fn is_prime(&self) -> bool {
				*self > 1 && primal::is_prime(*self as u64)
			}
		}
		)+
	};
}

macro_rules! snums {
    ($($ty:ident)+) => {
		$(
		impl SNumExt for $ty {
			fn negative(self) -> Option<Self> {
				(self < 0).then_some(self)
			}
		}
		)+
	};
}

nums! { i8 u8 i16 u16 i32 u32 i64 u64 isize usize }
snums! { i8 i16 i32 i64 isize }
