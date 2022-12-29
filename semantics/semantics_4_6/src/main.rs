fn main() {
    // loop
    loop {
        println!("loop forever!");
        break;
    }

    // while
    let mut x: i32 = 5;
    let mut done: bool = false;

    while !done {
        x += x  - 3;

        println!("{}",x);

        if x % 5 == 0 {
            done = true;    
        }
    }

    // for
    for x in 0..10 {
        println!("{}",x);
    }

    let a = 0..10;
    println!("test : {:?}",a);

    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}",i,j);
    }

    let mut x: i32 = 5;

    loop {
        x += x  - 3;

        println!("{}",x);

        if x % 5 == 0 {break;}
    }

    for x in 0..10 {
        if x % 2 == 0 {continue;}
        println!("{}",x);
    }

    // loop label
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {continue 'outer;}
            if y % 2 == 0 {continue 'inner;}
            if y == 5 {break 'inner;}
            println!("x: {}, y: {}",x,y);
        }
    }

}
