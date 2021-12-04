use crate::Idx;

newtype_index! {
    // Not sure if I want to support this syntax
    // It's only there so the serde derives can be added on
    #[derive(Default)]
    pub Bar
}

newtype_index!(Item);
newtype_index! {
    pub struct Foo {
        pub const B = 999;
        pub const C = 5;
    }
}

newtype_index! {
    #[derive(Default)]
    pub struct Qux {}
}

#[test]
fn test_macro_syntax() {
    newtype_index!(pub Bar);
    assert_eq!(Foo::B, Foo::new(999));
    assert_eq!(Foo::C, Foo::new(5));
    Bar::new(0);
}

#[test]
fn test_from_str() {
    newtype_index!(pub Bar);
    assert_eq!("5".parse::<Bar>().unwrap(), Bar::new(5));
}

#[test]
#[cfg(feature = "serde")]
fn test_serde_is_transparent() {
    let item = Item::new(42);
    assert_eq!(serde_json::to_string(&item).unwrap(), "42");
}
