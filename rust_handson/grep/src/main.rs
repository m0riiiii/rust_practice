use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut file = File::open(&args[1]).unwrap();
        let mut concat = String::new();

        file.read_to_string(&mut concat);
        println!("{:?}", concat);
    }
}
