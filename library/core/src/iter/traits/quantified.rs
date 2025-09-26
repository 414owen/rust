//! This module provides way to classify iterators and the knowledge we have
//! about their lengths at runtime.
//!
//! The most useful quantity is `Exact`, which means we know its exact size
//! at runtime.
//!
//! The second most useful quantity is `Infinite`, whose iterators will, for
//! example, when zipped with an `Exact`ly quantified iterator, produce another
//! `Exact`ly quantified iterator.

#[allow(missing_debug_implementations)]
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
/// A quantity for qhich ExactSizeIterator is blanket-defined
pub struct Exact {}

#[allow(missing_debug_implementations)]
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
/// A quantity for iterators whose `.next()` function will never return `None`
pub struct Infinite {}

// Implements a type-level function with two parameters and one output.
// Takes the name the function, and the pairs of inputs and ouputs.
//
// For example:
// ```
// quantify_fn_2 ! {
//   ChainQuantity;
//   Exact, Infinite => Infinite;
//   Infinite, Exact => Infinite;
// }
// ```
//
// Creates:
// ```
// trait ChainQuantity {
//     type Result;
// }
//
// impl ChainQuantity for (Exact, Infinite) {
//     type Result = Infinite;
// }
//
// impl ChainQuantity for (Infinite, Exact) {
//     type Result = Infinite;
// }
// ```

macro_rules! quantify_fn_2 {
    (
        $trait_name:ident,
        $($lhs:ty, $rhs:ty => $out:ty,)*
    ) => {
        #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
        pub trait $trait_name {
            #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
            type Result;
        }

        $(
            #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
            impl $trait_name for ($lhs, $rhs) {
                type Result = $out;
            }
        )*
    };
}

// As above, but for single-argument type-level functions
macro_rules! quantify_fn_1 {
    (
        $trait_name:ident,
        $($input:ty => $out:ty,)*
    ) => {
        #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
        pub trait $trait_name {
            #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
            type Result;
        }

        $(
            #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
            impl $trait_name for $input {
                type Result = $out;
            }
        )*
    };
}

pub(crate) use {quantify_fn_1, quantify_fn_2};

/// This is a trait whose sole purpose is to propagate information about the quality
/// of runtime size information of iterators.
/// It enables, for example, `Zip<A, B>` to implement `ExactSizeIterator` when either
/// `A`, and `B`, both implement `ExactSizeIterator`, or when one does, but the other
/// is infinite.
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
pub trait QuantifiedIterator: Iterator {
    /// This can either be `Exact`, if the exact size is known at runtime,
    /// `Infinite`, if `next()` will never return `None`, or `Finite`.
    ///
    /// In the case where you don't know which of these behaviours fits your iterator,
    /// don't implement this trait at all.
    #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
    type Quantity = Exact;

    /// Returns the exact remaining length of the iterator.
    ///
    /// The implementation ensures that the iterator will return exactly `len()`
    /// more times a [`Some(T)`] value, before returning [`None`].
    /// This method has a default implementation, so you usually should not
    /// implement it directly. However, if you can provide a more efficient
    /// implementation, you can do so. See the [trait-level] docs for an
    /// example.
    ///
    /// This function has the same safety guarantees as the
    /// [`Iterator::size_hint`] function.
    ///
    /// [trait-level]: ExactSizeIterator
    /// [`Some(T)`]: Some
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// // a finite range knows exactly how many times it will iterate
    /// let mut range = 0..5;
    ///
    /// assert_eq!(5, range.len());
    /// let _ = range.next();
    /// assert_eq!(4, range.len());
    /// ```
    #[inline]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn len(&self) -> usize
    where
        Self: QuantifiedIterator<Quantity = Exact>,
    {
        let (lower, upper) = self.size_hint();
        assert_eq!(upper, Some(lower));
        lower
    }

    /// Returns `true` if the iterator is empty.
    ///
    /// This method has a default implementation using
    /// [`ExactSizeIterator::len()`], so you don't need to implement it yourself.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(exact_size_is_empty)]
    ///
    /// let mut one_element = std::iter::once(0);
    /// assert!(!one_element.is_empty());
    ///
    /// assert_eq!(one_element.next(), Some(0));
    /// assert!(one_element.is_empty());
    ///
    /// assert_eq!(one_element.next(), None);
    /// ```
    #[inline]
    #[unstable(feature = "exact_size_is_empty", issue = "35428")]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
impl<I: QuantifiedIterator> QuantifiedIterator for &mut I {
    type Quantity = I::Quantity;
}
