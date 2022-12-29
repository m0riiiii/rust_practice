fn main() {
    type Name = String;
    let x: Name = "Hello".to_string();

    println!("{}", x);

    type Num = i32;
    let x: i32 = 5;
    let y: Num = 5;

    if x == y {
        println!("x == y");
    }

    use std::result;

    #[allow(dead_code)]
    enum ConcreteError {
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T, ConcreteError>;
}
