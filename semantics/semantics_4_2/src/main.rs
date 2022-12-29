fn main() {
    println!("{}",foo(3));
    print_number(5);
    print_sum(5,6);
    println!("{}",add_one(99));

    // 式対文
    let mut y = 5;
    let x = (y = 6);
    //
    println!("x: 空のtuple, y: {}",y);

    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    let f = add_one;

    let six = f(5);
    println!("{}",six);

    diverges();
}

fn foo(x: i32) -> i32 {
    return x;

    // このコードは走らない
    x + 1
}

fn print_number(x: i32){
    println!("x is {}",x);
}

// 型宣言をしないとエラー
//fn print_sum(x, y){
fn print_sum(x: i32, y: i32){
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    // ; をつけるとエラー
    //x + 1;
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
