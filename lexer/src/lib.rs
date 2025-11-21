use std::str::Chars;

#[derive(Debug)]
struct Reader<'a> {
    source: &'a str,
    source_iter: Chars<'a>,
    reader_pos: usize,
    token_pos: usize,

    is_first_char: bool,
    pointer_encountered: bool,
    starts_with_whitespace: bool,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a str) -> Self {
        Self{
            reader_pos: 0,
            token_pos: 0,
            pointer_encountered: false,
            starts_with_whitespace: false,
            is_first_char: true,
            source,
            source_iter: source.chars()
        }
    }
    pub fn reset_for_token_read(&mut self) {
        self.is_first_char = true;
        self.token_pos = 0;
        self.starts_with_whitespace = false;
        self.pointer_encountered = false;
    }
    pub fn is_eof(&self) -> bool {
        self.reader_pos >= self.source.len()
    }
    pub fn read_token(&mut self) {
        self.reader_pos += 1;
    }
}

pub fn get_tokens(source: &str) {
    let mut reader = Reader::new(source);
    while !reader.is_eof() {
        reader.reset_for_token_read();
        reader.read_token();
    }
}