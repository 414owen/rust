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
/// A quantity for iterators which are known to terminate, but whose exact length
/// isn't known at runtime
pub struct Finite {}

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
    type Quantity;
}

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
impl<I: QuantifiedIterator> QuantifiedIterator for &mut I {
    type Quantity = I::Quantity;
}
