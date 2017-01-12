

#[allow(unused_variables)]
fn main() {
    let x: i32 = 5;
    let y = x as i64;

    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;

    println!("{}", two_hundred);

    // ポインタキャスト
    let a = 300 as *const char;
    let b = a as u32;

    println!("{:?}", a);

    let a = [0u8, 0u8, 0u8, 0u8];
    use std::mem as memory;
    
    unsafe {
        let b = memory::transmute::<[u8;4], u32>(a);
        println!("{:?}", b);
    }
}
