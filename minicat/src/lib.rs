use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};

pub fn run<W: Write>(w: &mut W, filenames: Vec<String>) -> io::Result<()> {
    for name in filenames {
        write!(w, "{}", open_and_read_file(&name)).expect("fail to write");
    }
    Ok(())
}

fn open_and_read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_run() {
        use crate::run;
        let expected = b"foo\nbar\n";
        let mut buf = Vec::<u8>::new();
        let result = run(
            &mut buf,
            vec![
                "./tests/data/example1.txt".to_string(),
                "./tests/data/example2.txt".to_string(),
            ],
        );
        assert!(result.is_ok());
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_open_and_read_file() {
        use crate::open_and_read_file;
        let expected = "foo\n";
        let result = open_and_read_file("./tests/data/example1.txt");
        assert_eq!(expected, result);
    }
}
