use common::CommentType;
use common::Token;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
struct ReadTokenResult {
    should_skip_next: bool,
    continue_reading: bool,
    token: Token,
}

impl ReadTokenResult {
    pub fn new() -> Self {
        Self {
            should_skip_next: false,
            continue_reading: true,
            token: Token::Eof,
        }
    }
}

#[derive(Debug)]
struct Reader<'a> {
    source: &'a str,
    source_iter: Peekable<Chars<'a>>,
    reader_pos: usize,
    token_pos: usize,

    is_first_char: bool,
    pointer_encountered: bool,
    starts_with_whitespace: bool,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader_pos: 0,
            token_pos: 0,
            pointer_encountered: false,
            starts_with_whitespace: false,
            is_first_char: true,
            source,
            source_iter: source.chars().peekable(),
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
    fn peek(&mut self) -> &char {
        self.source_iter.peek().unwrap_or(&'\0')
    }
    fn skip_whitespace(&mut self) {
        while self.source_iter.next().unwrap_or('\0').is_whitespace() {
            self.reader_pos += 1;
        }
    }
    fn read_first_char(&mut self) -> ReadTokenResult {
        let mut result = ReadTokenResult::new();
        self.skip_whitespace();
        match self.source_iter.next() {
            Some(chr) => {
                if !chr.is_whitespace() {
                    if chr == '{' {
                        result.token = Token::Comment(CommentType::CurlyBrackets)
                    } else if chr == '[' {
                        result.continue_reading = false;
                        result.token = Token::OpenSquareBracket
                    } else if chr == ']' {
                        result.continue_reading = false;
                        result.token = Token::CloseSquareBracket
                    } else if chr == '(' {
                        if *self.peek() == '*' {
                            result.token = Token::Comment(CommentType::Parenthesis)
                        } else {
                            result.continue_reading = false;
                            result.token = Token::OpenParenthesis
                        }
                    } else if chr == ')' {
                        result.continue_reading = false;
                        result.token = Token::CloseParenthesis
                    } else if chr == '\'' {
                        result.token = Token::CharLiteral(String::new())
                    } else if chr.is_alphabetic() || chr == '_' {
                        result.token = Token::Identifier(chr.to_string());
                        let next = *self.peek();
                        result.continue_reading = next == '_' || next.is_alphanumeric();
                    } else if chr.is_ascii_digit() {
                        result.token = Token::IntLiteral(chr.to_string());
                        result.continue_reading = self.peek().is_ascii_digit();
                    } else if common::SYMBOLS.contains(&chr) {
                        result.token = Token::Symbol(chr.to_string());
                        let next = *self.peek();
                        // Check if the next char is possibly the beginning of a pointer/deref or string literal.
                        if (chr != '^' && next == '^') || (chr != '\'' && next == '\'') {
                            result.continue_reading = false;
                        }
                        // Check if the symbol ends in the next char.
                        if !common::SYMBOLS.contains(&next) {
                            result.continue_reading = false;
                        }
                    } else if common::OPERATORS.contains(&chr) {
                    }
                }
            }
            None => result.continue_reading = false,
        }
        self.reader_pos += 1;
        result
    }
    pub fn read_token(&mut self) {
        self.reset_for_token_read();
        self.read_first_char();
    }
}

pub fn get_tokens(source: &str) {
    let mut reader = Reader::new(source);
    while !reader.is_eof() {
        reader.read_token();
    }
}
