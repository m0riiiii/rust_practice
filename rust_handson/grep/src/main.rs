use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let mut file = File::open(&args[1]).unwrap();
        let mut concat = String::new();

        file.read_to_string(&mut concat);
        let strings: Vec<&str> = concat.split('\n').collect();

        for (line_count,line) in strings.iter().enumerate() {
            if line.contains(&args[2]) {
                println!("{} Ln.{}: {}", &args[1], line_count, line);
            }
        }
    }
}
