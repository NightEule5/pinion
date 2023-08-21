// SPDX-License-Identifier: Apache-2.0

//! Extensions for pointers. Requires the `unsafe` feature.

#![cfg(feature = "unsafe")]

mod sealed {
	pub trait SealedPtr { }
	impl<T> SealedPtr for *const T { }
	impl<T> SealedPtr for *mut   T { }
}

pub trait PtrExt: Sized + sealed::SealedPtr {
	/// Returns `None` if the pointer is null, or wraps it in `Some` if it points
	/// to a value.
	fn non_null(self) -> Option<Self>;
}

impl<T> PtrExt for *const T {
	fn non_null(self) -> Option<Self> {
		self.is_null().then_some(self)
	}
}

impl<T> PtrExt for *mut T {
	fn non_null(self) -> Option<Self> {
		self.is_null().then_some(self)
	}
}
