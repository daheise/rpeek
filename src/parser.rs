use std::fs::File;

struct SourceText {
    at_eof: bool,
    block: usize,
    blocks: [[char; 2]; 1024],
    eof_count: usize,
    name: String,
    next: usize,
    column: usize,
    line: usize,
    initialized: bool,
    stream: File,
}

trait Parser{
    fn new(name: &str) -> Self;
    fn consume(&self) -> Result<char, &'static str>;
    fn consume_n(&self, n: usize) ->  Result<&str, &'static str>;
    fn consume_whitespace(&self) -> Result<(),  &'static str>;
    fn eof(&self) -> bool;
    fn loc(&self) -> (usize, usize);
    fn peek(&self) -> Result<char, &'static str>;
    fn peek_n(&self, n: usize) -> Result<&str, &'static str>;
    fn peek_str(&self, s_str: &str) -> Result<bool, &'static str>;
    fn consume_str(&self, s_str: &str) -> Result<bool, &'static str>;
}
