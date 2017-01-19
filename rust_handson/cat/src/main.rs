extern crate getopts;

use getopts::Options;
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: String) -> Result<String, io::Error> {
    let mut file = try!(File::open(filename));
    let mut concat = String::new();
    try!(file.read_to_string(&mut concat));

    Ok(concat)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("n","number","print line number");
    opts.optflag("h","help","print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        println!("{}", opts.usage(""));
        return;
    };

    if args.len() > 1 {
        let mut count = 0;
        for file in &args[1..] {
            if &file[0..1] == "-" {
                continue;
            }
            println!("{}", match read_file(file.clone()) {
                Ok(concat) => concat.lines()
                                    .enumerate()
                                    .map(|(_,line)| if matches.opt_present("n") {
                                            count += 1;
                                            format!("{}: {}",(count),line)
                                        } else { 
                                            format!("{}", line)
                                            })
                                    .fold(String::new(), |result, line| format!("{}\n{}",result,line)),
                Err(_) => format!("{} is not exist", file),
            });
        }
    }
}
