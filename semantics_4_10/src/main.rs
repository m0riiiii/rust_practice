use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

fn main() {
    let x = 5;
    // error
    //x = 6;

    let mut x = 5;
    x = 6;

    println!("x: {}",x);

    {
        let y = &mut x;
        *y = 7;
        //let z = &mut y;

        println!("y: {}, *y: {}",y,*y);
    }

    {
        let mut y = &mut x;
        let z = &mut y;
        **z = 10;

        println!("z: {}, *z: {}, **z: {}",z,*z,**z);
    }

    let (mut x,y) = (5,6);
    x = 11;

    println!("x: {}",x);

    // 外側のミュータビリティ
    let x = Arc::new(5);
    let y = x.clone();

    // 内側のミュータビリティ
    let x = RefCell::new(42);
    let y = x.borrow_mut();
    // this is error! panic! occur
    //let z = x.borrow_mut();

    // 構造体のミュータビリティ
    struct Point {
        x: i32,
        y: i32,
    }

    let mut a = Point {x:5, y:6};
    a.x = 10;

    let b = Point {x:5, y:6};
    //b.x = 10;

    struct Point2 {
        x: i32,
        y: Cell<i32>,
    }

    let point = Point2 { x:5, y:Cell::new(6) };
    // フィールドれべるのミュータビリティができる
    point.y.set(7);

    println!("point.y: {}",point.y.get());
}
