use std::net::TcpStream;

fn take_slice(slice: &str) {
    println!("Got: {}",slice);
}

fn main() {
    let greeting: &'static str = "Hello there.";

    let s = "foo
             bar";
    assert_eq!("foo\n             bar",s);

    let s = "foo\
             bar";
    assert_eq!("foobar",s);

    let mut s: String = "Hello".to_string();
    println!("{}",s);

    s.push_str(", world.");
    println!("{}",s);

    take_slice(&s);

    TcpStream::connect("192.168.0.1.3000");

    let add_string = "192.168.0.1:3000".to_string();
    TcpStream::connect(&*add_string);

    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
        print!("{},",b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{},",c);
    }

    println!("");

    println!("second char is {:?}",hachiko.chars().nth(1));

    // slicing
    let dog = "hachiko";
    let hachi = &dog[0..5];
    println!("{}",hachi);

    let dog = "忠犬ハチ公";
    //this is error!
    //let hachi = &dog[0..2];

    let hello = "Hello ".to_string();
    let world = "world!".to_string();

    let hello_world = hello + &world;


}
