use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut file = try!(File::open(args[1]));
        let mut concat = String::new();

        try!(file.read_to_string(&mut concat));
        println!("{:?}", concat);
    }
}
