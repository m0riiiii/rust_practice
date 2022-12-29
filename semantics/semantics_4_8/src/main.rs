fn main() {
    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<i32> = vec![1,2,3,4];

    let answer = foo(&v1,&v2);

    println!("in main v1: {:?}, v2: {:?}, answer: {}",v1,v2,answer);

    // mut参照
    let mut x = 5;
    {
        let y = &mut x;
        // this is error! cannot use `+=` on type `&mut {integer}`
        //y += 1;
        //*y += 1;
    }
    println!("{}",x);

    println!("{}",x);

    // 参照(&T) ミュータブルな参照（&mut T）

    let mut x = vec![1,2,3];
    let y = vec![1,2,3,4];

    setter(&mut x,y);

    println!("{:?}", x);
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // this is error! 参照はimmutable
    //v1.push(5);

    println!("in function v1: {:?}, v2: {:?}",v1,v2);

    42
}

fn setter(x: &mut Vec<i32>, y: Vec<i32>) {
    *x = y;
}
