use std::ops::Deref;
use std::rc::Rc;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.value
    }
}

fn foo(s: &str) {
    println!("{}", s);
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);

    // Derefによる型強制
    let owned = "Hello".to_string();
    let counted = Rc::new(owned);

    foo(&counted);

    struct Foo;
    impl Foo {
        fn foo(&self) { println!("Foo!"); }
    }

    let f = &&Foo;
    f.foo();
}
