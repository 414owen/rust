// issue #21405
struct Foo;

fn foo<F>(f: F) where F: FnMut(Foo) {}

fn main() {
    foo(|s| s.is_empty());
    //~^ ERROR the method `is_empty` exists for struct `Foo`, but its trait bounds were not satisfied [E0599]
}
