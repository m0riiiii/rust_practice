//fn sum(n: i32, acc: i32) -> i32 {
//    if n == 0 {acc} else {sum(n-1, acc+1)}
//}

fn main() {
    // rustc -O でTCOが可能
    //println!("{}", sum(100000000, 0));

    println!("Hello, world!");

    let mut name = "m0riiii";
    println!("hello, {}", name);
    
    name = "change";
    println!("hello, {}", name);

    let vec = vec![1,2,3];
    println!("all vec is {:?}", vec);

    println!("[reprint]all vec is {:?}", vec);

    println!("1 + 1 = {}", add(1,1));
    println!("13 + 23 = {}", add(13,23));

    println!("1 - 1 = {}", sub(1,1));

    println!("2 * 4 = {}", times(2,4));

    let a: () = f();

    let mut a = [0,1,2,3,4,5];
    a[2] = 100;
    
    println!("{:?}", a);
    println!("a.len() = {}", a.len());

    let middle = &mut a[1..4];

    println!("{:?}", middle);
    println!("middle.len() = {}", middle.len());

    println!("{:?}", [fib(1),fib(2),fib(3),fib(4),fib(5),fib(6)]);

    let mut b: [i32;3] = [0;3];
    //b = [1,2,3];
    b[0] = 1;

    println!("{:?}", b);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn times(a: i32, b: i32) -> i32 {
    a * b
}

fn f(){}

fn fib(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n -2)
    }
}
