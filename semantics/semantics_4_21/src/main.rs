enum Test {
    Unit,
    None,
}

fn main() {
    // if let
    let option = Some("test".to_string());
    match option {
        Some(x) => { foo(x) },
        None => {},
    }

    let option = Some("test".to_string());
    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    let option = Some("test".to_string());
    if let Some(x) = option {
        foo(x);
    }

    let option = Test::None;
    if let Test::Unit = option {
        foo("test".to_string());
    } else {
        bar();
    }

    // while let
    let mut v = vec![1, 3, 5, 7, 11];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }
}

fn foo(x: String) {
    println!("match! {}", x);
}

fn bar() {
    println!("not match");
}
