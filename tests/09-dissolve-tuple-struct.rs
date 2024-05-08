//! Try with generics and references.

use derive_getters::Dissolve;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Dissolve)]
struct SimpleUnnamed(u64, i64);

impl SimpleUnnamed {
    pub fn manual_dissolve(self) -> (u64, i64) {
        (self.0, self.1)
    }
}

fn main() {
    let su = SimpleUnnamed(44,-100);
    let (a, b) = su.dissolve();

    assert_eq!(44, a);
    assert_eq!(-100, b);
}

/*
#[derive(Getters, Dissolve)]
struct MultiAnnotated<'a, 'b, 'c, T> {
    v1: &'a str,
    v2: &'b [u8],
    v3: &'c T,
    owned: String,
}

impl<'a, 'b, 'c, T> MultiAnnotated<'a, 'b, 'c, T> {
    pub fn new(v1: &'a str, v2: &'b [u8], v3: &'c T, owned: String) -> Self {
        MultiAnnotated { v1, v2, v3, owned }
    }
}

#[derive(Getters, Dissolve)]
#[dissolve(rename = "unmake")]
struct PolyAnnotated<'a, 'b, 'c, T> {
    v1: &'a str,
    v2: &'b [u8],
    v3: &'c T,
    owned: String,
}

impl<'a, 'b, 'c, T> PolyAnnotated<'a, 'b, 'c, T> {
    pub fn new(v1: &'a str, v2: &'b [u8], v3: &'c T, owned: String) -> Self {
        PolyAnnotated { v1, v2, v3, owned }
    }

    pub fn dissolve(self) -> String {
        self.owned
    }
}

fn main() {
    let buffer: [u8; 12] = [88; 12];
    let gt = ConcreteType { a: 44, b: -100 };
    let ma = MultiAnnotated::new("Hi", &buffer, &gt, "Another".to_owned());

    let (v1, v2, v3, owned) = ma.dissolve();
    assert!(v1 == "Hi");
    assert!(v2 == &buffer);
    assert!(*v3 == gt);
    assert!(owned == "Another");

    let pa = PolyAnnotated::new("Hi", &buffer, &gt, "Another".to_owned());
    let (v1, v2, v3, owned) = pa.unmake();
    assert!(v1 == "Hi");
    assert!(v2 == &buffer);
    assert!(*v3 == gt);
    assert!(owned == "Another");
}
*/
