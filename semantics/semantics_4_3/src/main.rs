fn main() {
    // boolean
    let x = true;
    let y: bool = false;
    println!("x: {},y: {}",x,y);

    // char
    let x = 'x';
    let two_hearts = '💕';
    println!("x: {},two_hearts: {}",x,two_hearts);

    // 整数型
    let x = 42;
    let y = 1.0;
    println!("x: {},y: {}",x,y);

    // 配列
    let a: [i32; 3] = [1,2,3];
    let mut m = [1,2,3];
    println!("a: {:?},m: {:?}",a,m);

    let a = [0;20];
    println!("a is {:?}\na has {} elements",a,a.len());

    let names: [&str;3] = ["Graydon","Brian","Niko"];
    println!("The second name is: {}",names[1]);

    // slice
    let a = [0,1,2,3,4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("complete: {:?},middle :{:?}",complete,middle);

    // tuple
    let x:(i32,&str) = (1,"hello");

    let (a,b,c) = (1,2,3);

    println!("x: {:?}\na is {}",x,a);

    let tuple = (1,2,3);
    let d = tuple.0;
    let e = tuple.1;
    println!("d: {},e: {}",d,e);

    fn foo(x: i32) -> i32 {x}
    let x: fn(i32) -> i32 = foo;
    println!("foo(2): {}",x(2));

}
