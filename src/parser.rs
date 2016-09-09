use std::path::Path;
use std::fs::File;

pub struct SourceText {
    at_eof: bool,
    eof_count: usize,
    name: String,
    column: usize,
    line: usize,
    stream: File,
}

pub trait Parser{
    fn new(path: &Path) -> Self;
    //fn consume(&self) -> Result<char, &'static str>;
    //fn consume_n(&self, n: usize) ->  Result<&str, &'static str>;
    //fn consume_whitespace(&self) -> Result<(),  &'static str>;
    //fn eof(&self) -> bool;
    //fn loc(&self) -> (usize, usize);
    //fn peek(&self) -> Result<char, &'static str>;
    //fn peek_n(&self, n: usize) -> Result<&str, &'static str>;
    //fn peek_str(&self, s_str: &str) -> Result<bool, &'static str>;
    //fn consume_str(&self, s_str: &str) -> Result<bool, &'static str>;
}

impl Parser for SourceText{
    fn new(path: &Path) -> SourceText {
        // TODO: Error checking
        let f = File::open(path).unwrap();
        // TODO: Lots of error checking
        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
        print!("{}", filename);

        SourceText {
            at_eof: false,
            eof_count: 0,
            name: filename,
            column: 0,
            line: 0,
            stream: f
        }
    }
}
