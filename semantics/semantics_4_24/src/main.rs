trait Foo {
    fn f(&self);
    fn foo() -> i32;
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Baz {
    fn foo() -> i32 {
        20
    }
}

impl Foo for Baz {
    fn f(&self) { println!("Baz's impl of Foo"); }
    fn foo() -> i32 {
        10
    }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz's impl of Bar"); }
}

fn main() {
    let b = Baz;

    Foo::f(&b);
    Bar::f(&b);

    assert_eq!(10, <Baz as Foo>::foo());
    assert_eq!(20, Baz::foo());
}
