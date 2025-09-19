/// An iterator that will always produce more items.
///
/// This is useful in order to have more types implement `ExactSizeIterator`.
///
/// ```
/// [1, 2, 3].zip(repeat(4))
/// ```
///
/// This is an iterator with a known exact size of three.
///
/// In order to propagate this information to the compiler, we
/// have:
///
/// ```text
/// impl<A: Clone> QuantifiedIterator for Repeat<A> {}
/// impl<A: Clone, B: ExactSizeIterator> ExactSizeIterator for Zip<Repeat<A>, B> {}
/// impl<A: ExactSizeIterator, B: Clone> ExactSizeIterator for Zip<A, Repeat<B>> {}
/// ```
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
pub struct Exact {}
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
pub struct Finite {}
#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
pub struct Infinite {}

#[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
pub trait QuantifiedIterator: Iterator {
    #[stable(feature = "infinite_iterator_trait", since = "CURRENT_RUSTC_VERSION")]
    type Quantity;
}
