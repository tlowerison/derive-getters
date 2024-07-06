use derive_getters::Getters;

#[derive(Getters)]
pub struct Auto {
    u32: u32,
    string: String,
}

#[test]
fn test_auto_copy_getters() {
    let u32 = 0;
    let string = "Hello".to_string();
    let auto = Auto {
        u32,
        string: string.clone(),
    };

    assert_eq!(u32, auto.u32()); // getter returns a copy
    assert_eq!(&string, auto.string()); // getter returns a reference
}
