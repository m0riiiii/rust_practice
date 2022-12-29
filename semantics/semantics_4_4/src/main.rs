fn main() {

    // 行コメント
    let x = 5; // this is also a line comment.

    println!("add_one(5) is {}",add_one(x));

}

/// 与えられた数値に1を加える
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
