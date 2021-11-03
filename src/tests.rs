use crate::Idx;

newtype_index!(Item);
newtype_index! {
    pub struct Foo {
        pub const B = 999;
        pub const C = 5;
    }
}

#[test]
fn test_macro_syntax() {
    newtype_index!(pub Bar);
    assert_eq!(Foo::B, Foo::new(999));
    assert_eq!(Foo::C, Foo::new(5));
    Bar::new(0);
}
