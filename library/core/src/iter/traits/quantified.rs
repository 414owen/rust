//! An iterator that will always produce more items.
//!
//! This is useful in order to have more types implement `ExactSizeIterator`.
//!
//! ```
//! [1, 2, 3].zip(repeat(4))
//! ```
//!
//! This is an iterator with a known exact size of three.
//!
//! In order to propagate this information to the compiler, we
//! have:
//!
//! ```text
//! impl<A: Clone> QuantifiedIterator for Repeat<A> {}
//! impl<A: Clone, B: ExactSizeIterator> ExactSizeIterator for Zip<Repeat<A>, B> {}
//! impl<A: ExactSizeIterator, B: Clone> ExactSizeIterator for Zip<A, Repeat<B>> {}
//! ```

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
#[derive(Debug)]
/// A quantity for qhich ExactSizeIterator is blanket-defined
pub struct Exact {}

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
/// A quantity for iterators which are known to terminate, but whose exact length
/// isn't known at runtime
#[derive(Debug)]
pub struct Finite {}

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
/// A quantity for iterators whose `.next()` function will never return `None`
#[derive(Debug)]
pub struct Infinite {}

// Implements a type-level function with two parameters and one output
// Takes the name the function, and the truth table.
//
// For example:
// ```
// type_fn! {
//   ChainQuantity;
//   Exact, Finite => Finite;
//   Exact, Infinite => Infinite;
// }
// ```
//
// Takes:
// ```
// trait ChainQuantity {
//     type Result;
// }
//
// impl ChainQuantity for (Exact, Finite) {
//     type Result = Finite;
// }
//
// impl ChainQuantity for (Exact, Infinite) {
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
