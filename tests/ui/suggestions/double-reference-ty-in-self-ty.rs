// issue#135863

struct A;

impl A {
    fn len(self: &&A) {}
}

fn main() {
    A.len();
    //~^ ERROR: the method `len` exists for struct `A`, but its trait bounds were not satisfied [E0599]
}
