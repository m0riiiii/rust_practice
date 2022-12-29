fn main() {
    // 変数束縛
    {
        let x = 5;
        println!("x:{}",x);
    }

    // パターン
    {
        let (x,y) = (1,2);
        println!("x:{},y:{}",x,y);
    }

    // 型アノテーション
    {
        let x: i32 = 5;
        println!("x:{}",x);
    }

    // mutable
    {
        let mut x = 5;
        x = 10;
        println!("x:{}",x);
    }

    // 初期化（警告）
    {
        let x: i32;
        println!("Hello World!");
    }

    // 初期化（エラー）
    {
        let x: i32;
        //println!("The value of x is: {}",x);
    }

    // scope
    {
        let x: i32 = 17;
        {
            let y: i32 = 3;
            println!("The value of x is {} and value of y is {}",x,y);
        }
        //this is error
        //println!("The value of x is {} and value of y is {}",x,y);
    }

    // shadowing
    {
        let x: i32 = 8;
        {
            println!("{}",x);
            let x = 12;
            println!("{}",x);
        }
        println!("{}",x);
        let x = 42;
        println!("{}",x);
    }

    // shadowing and mutable
    {
        let mut x: i32 = 1;
        x = 7;
        // this is error
        //x = "error!";
        let x = x;

        let y = 4;
        let y = "I can also be bound to text!";

        println!("x: {}, y: {}",x,y)
    }
}
