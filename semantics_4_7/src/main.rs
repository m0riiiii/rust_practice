fn main() {

    // 変数束縛v = 束縛されているものの「所有権を持つ」

    // move semantics
    let v = vec![1,2,3];
    let _v2 = v;

    // this is error! 「value used after move」
    //println!("v[0] is: {}",v[0]);

    let v3 = vec![1,2,3];

    let v4 = take(v3);

    // this is error! 
    //println!("v3[0] is :{}",v3[0]);
    println!("v4 is ok {:?}",v4);

    // i32はCopy traitを実装
    // 所有権の変更はしない
    let a = 5;
    let _y = double(a);
    println!("{}",a);

    // boolはCopy traitを実装
    // 所有権の変更はしない
    let a = true;
    let _y = change_truth(a);
    println!("{}",a);
}

fn take(v: Vec<i32>) -> Vec<i32> {
    println!("{:?}",v);

    v
}

fn double(x: i32) -> i32 {
    x * 2    
}

fn change_truth(x :bool) -> bool {
    !x
}
