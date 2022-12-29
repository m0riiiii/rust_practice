struct Point {
    x: i32,
    y: i32,
}

struct Person {
    name: Option<String>,
}

enum OptionTuple {
    Value(i32, i32, i32),
    Missing,
}

enum OptionalInt {
    Value(i32),
    Missing,
}

fn main() {
    let x = 'x';
    let c = 'c';

    match c {
        '1' => println!("1"),
        a => println!("x: {} c:{}",x,c),
    }

    println!("x: {}",x);

    // 複式パターン
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        a => println!("other: {}",a),
    }

    // デストラクチャリング
    let origin = Point {x:0, y:0 };

    match origin {
        Point {x:x1, ..} => println!("x is {}",x1)
    }

    // 束縛の無視
    let x = OptionTuple::Value(5,-2,3);

    match x {
        OptionTuple::Value(..) => println!("Got a tuple!"),
        OptionTuple::Missing => println!("No such luck."),
    }

    // ref & ref mut
    let x = 5;

    match x {
        ref r => println!("got a reference to {}",r),
    }

    let mut x = 5;

    match x {
        ref mut mr => { 
                           *mr = 6;
                           println!("Got a mutable reference to {}",mr)
                      },
    }

    // range

    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    // 束縛
    let x = 1;

    match x {
        e @ 1 ... 5 => println!("got a range element {}",e),
        _ => println!("anything"),
    }

    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), ..}) => println!("{:?}",a),
        _ => {}
    }

    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(_) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck"),
    }
}
