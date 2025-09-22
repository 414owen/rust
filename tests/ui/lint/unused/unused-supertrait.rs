#![deny(unused_must_use)]

fn it() -> impl ExactSizeIterator<Item = ()> {
    use std::iter::{QuantifiedIterator, Exact};
    let x: Box<dyn QuantifiedIterator<Item = (), Quantity = Exact>> = todo!();
    x
}

fn main() {
    it();
    //~^ ERROR unused implementer of `Iterator` that must be used
}
