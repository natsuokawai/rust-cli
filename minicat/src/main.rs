use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1..];
    if filename.len() == 0 {
        eprintln!("{}: file name not given.", args[0]);
        process::exit(1);
    }
    println!("{:?}", filename);
}
