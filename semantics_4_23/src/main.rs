fn main() {
    let plus_one = | x: i32| x + 1;
    assert_eq!(2, plus_one(1));

    let plus_two = |x| {
        let mut result: i32 = x;

        result += 1;
        result += 1;

        result
    };

    assert_eq!(4, plus_two(2));

    fn  _plus_one_v1   (x: i32) -> i32 { x + 1 }
    let _plus_one_v2 = |x: i32| -> i32 { x + 1 };
    let _plus_one_v3 = |x: i32|          x + 1  ;
    // this is error!
    //let _plus_one_v4 = |x|               x + 1  ;

    let mut num = 5;
    num += 1;
    let plus_num = |x: i32| x + num;

    // already borrowed
    //let y = &mut num;
    let _y = &num;

    assert_eq!(11, plus_num(5));


    // move closure
    let num = 5;
    let owns_num = move |x: i32| num + x;
    println!("{}", owns_num(100));

    let mut num = 5;

    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }

    assert_eq!(10, num);

    let mut num = 5;
    
    {
        // move closure はコピーの所有権を取得する
        let mut add_num = move |x: i32| num += x;
        add_num(5); 
    }

    assert_eq!(5, num);

    // closure as args
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    // 関数ポインタとクロージャ
    fn call_with_two(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(2)
    }

    fn add_one(i: i32) -> i32 {
        i + 1
    }

    let f = add_one;
    let answer  = call_with_two(&f);
    assert_eq!(3, answer);

    // return closure
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;
        Box::new(move |x| x + num)
    }

    let f = factory();
    let answer = f(1);

    assert_eq!(6, answer);

}
