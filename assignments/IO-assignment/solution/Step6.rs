use url::Url;
use std::io::{Read, BufReader, BufRead, Lines};
use std::fs::File;

fn parse_line(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(u) => Some(u),
        Err(e) => None
    }
}

fn main() {
    let open_result = File::open("src/lib/content.txt");

    let file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut buf_reader = BufReader::new(file).lines();

    for line in buf_reader {

        let line = match line {
            Ok(content) => content,
            Err(e) => panic!("Problem reading the file: {:?}", e),
        };

        let url = parse_line(line);

        match url {
            Some(line) => println!("{}", line),
            None => continue
        }
    }
}

#[test]
fn correct_url() {
    assert!(parse_url("https://example.com").is_some())
}

#[test]
fn no_url() {
    assert!(parse_url("abcdf").is_none())
}
