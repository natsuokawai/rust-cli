extern crate minicat;

use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("{}: file name not given.", args[0]);
        process::exit(1);
    }
    let filenames: Vec<String> = args[1..].to_vec();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    minicat::run(&mut stdout, filenames).expect("failed to cat");
}
