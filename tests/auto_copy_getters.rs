#[test]
#[cfg(feature = "auto_copy_getters")]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/autocopy/01-auto-copy.rs");
}
