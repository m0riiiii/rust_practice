fn main() {

    let x = 4;

    if x == 5 {
        println!("x is 5 !");
    } else if x == 6 {
        println!("x is 6 !");
    }else {
        println!("x is not {{5 or 6}} !");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };

    println!("y: {:?}",y);
}
