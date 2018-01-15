extern crate rustd;

use std::ops::Deref;

fn main() {
    let x = 5;
    let y = &x;
    let z = Boxy::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

struct Boxy<T>(T);

impl<T> Boxy<T> {
    fn new(t: T) -> Boxy<T> {
        Boxy(t)
    }
}

impl<T> Deref for Boxy<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
