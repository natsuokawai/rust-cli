use std::fs::File;
use std::io::prelude::*;

fn open_and_read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

#[cfg(test)]
mod tests {
    use crate::open_and_read_file;

    #[test]
    fn test_open_and_read_file() {
        let expected = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
        let acutually = open_and_read_file("./tests/data/example.txt");
        assert_eq!(expected, acutually);
    }
}
