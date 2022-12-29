trait Foo{
    type N;
}

impl Foo for str {
    type N = &'static str;
}

struct Bar<T: ?Sized> {
    f: T,
}

fn main() {
    println!("Hello, world!");
}
