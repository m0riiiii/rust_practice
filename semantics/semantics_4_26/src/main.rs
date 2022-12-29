const N: i32 = 5;
static mut NAME: &'static str = "Steve";

fn main() {
    println!("{}", N);

    unsafe {
        println!("{}", NAME);
       
        NAME = "Change";
        println!("{}", NAME);
    }

}
