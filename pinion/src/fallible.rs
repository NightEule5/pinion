// SPDX-License-Identifier: Apache-2.0

//! Extensions for options and results.

mod sealed {
	pub trait SealedOptionExt { }
	pub trait SealedResultExt { }
	impl<T> SealedOptionExt for Option<T> { }
	impl<T, E> SealedResultExt for Result<T, E> { }
}

pub trait OptionExt<T>: sealed::SealedOptionExt {
	/// Inserts `value` into the option if it is [`None`]. Behavior is the same as
	/// [`get_or_insert`][], except no mutable reference is returned.
	///
	/// [`get_or_insert`]: Option::get_or_insert
	fn populate(&mut self, value: T);
	/// Inserts a value computed by `f` into the option if it is `None`. Behavior
	/// is the same as [`get_or_insert_with`][], except no mutable reference is
	/// returned.
	///
	/// [`get_or_insert_with`]: Option::get_or_insert_with
	fn populate_with(&mut self, f: impl FnOnce() -> T);
	/// Inserts the default value into the option if it is [`None`]. Behavior is the
	/// same as [`get_or_insert_default`][], except no mutable reference is returned.
	///
	/// [`get_or_insert_default`]: Option::get_or_insert_default
	fn populate_default(&mut self) where T: Default {
		self.populate_with(T::default)
	}

	/// Maps the option's contained value into type `R` implementing the [`From`]
	/// trait. Shorthand for `map(Into::into)`.
	fn map_into<R: From<T>>(self) -> Option<R>;
	/// Maps the option's contained value into a string. Shorthand for
	/// `as_ref().map(ToString::to_string)`.
	fn map_to_string(self) -> Option<String> where T: ToString;
}

impl<T> OptionExt<T> for Option<T> {
	fn populate(&mut self, value: T) {
		if let None = *self {
			*self = Some(value);
		}
	}

	fn populate_with(&mut self, f: impl FnOnce() -> T) {
		if let None = *self {
			*self = Some(f());
		}
	}

	fn map_into<R: From<T>>(self) -> Option<R> { self.map(R::from) }

	fn map_to_string(self) -> Option<String> where T: ToString {
		Some(self?.to_string())
	}
}

pub trait ResultExt<T, E>: sealed::SealedResultExt {
	/// Drops a contained [`Ok`] value, leaving unit in its place. Useful in cases
	/// where you call a function that returns an [`Ok`] value you don't care about,
	/// and need to "flick it away" to return unit:
	///
	/// ```no-run
	/// fn main() -> Result<(), Error> {
	/// 	// ugly
	/// 	do_a_thing()?;
	/// 	Ok(())
	///
	/// 	// better
	/// 	do_a_thing().flick()
	/// }
	///
	/// fn do_a_thing() -> Result<bool, Error> {
	/// 	// ...
	/// }
	/// ```
	fn flick(self) -> Result<(), E>;

	/// Maps a contained [`Ok`] value into type `R` implementing the [`From`] trait.
	/// Shorthand for `map(Into::into)`.
	fn map_into<R: From<T>>(self) -> Result<R, E>;
	/// Maps a contained [`Ok`] value into a string.
	fn map_to_string(self) -> Result<String, E> where T: ToString;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
	fn flick(self) -> Result<(), E> { self?; Ok(()) }

	fn map_into<R: From<T>>(self) -> Result<R, E> { self.map(R::from) }

	fn map_to_string(self) -> Result<String, E> where T: ToString {
		Ok(self?.to_string())
	}
}
