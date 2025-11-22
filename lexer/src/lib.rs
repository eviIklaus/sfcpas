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
    prev_char: Option<char>,
    current_char: Option<char>,

    is_first_char: bool,
    pointer_encountered: bool,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader_pos: 0,
            pointer_encountered: false,
            is_first_char: true,
            prev_char: None,
            current_char: None,
            source,
            source_iter: source.chars().peekable(),
        }
    }
    pub fn reset_for_token_read(&mut self) {
        self.is_first_char = true;
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
    fn read_next_char(&mut self) -> Option<char> {
        self.reader_pos += 1;
        self.prev_char = self.current_char;
        self.current_char = self.source_iter.next();
        self.current_char
    }
    fn read_first_char(&mut self) -> ReadTokenResult {
        let mut result = ReadTokenResult::new();
        self.skip_whitespace();
        match self.read_next_char() {
            None => result.continue_reading = false,
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
                        // Check if a single quote is in the string literal
                        result.token = Token::StringLiteral(String::new());
                        if *self.peek() == '\'' {
                            let Token::StringLiteral(mut val) = result.token else {
                                panic!("Impossible! It was set as a string literal before.")
                            };
                            val.push_str("\'");
                            result.token = Token::StringLiteral(val)
                        }
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
                        result.token = Token::Operator(chr.to_string());
                        // Check if it's an assignment operator or if the operator ends in the next char.
                        if chr == '=' || !common::OPERATORS.contains(&chr) {
                            result.continue_reading = false;
                        }
                    }
                }
            }
        }
        result
    }
    fn read_the_rest(&mut self, mut result: ReadTokenResult) -> ReadTokenResult {
        self.reader_pos += 1;
        let chr = match self.read_next_char() {
            Some(chr) => chr,
            None => {
                result.continue_reading = false;
                return result;
            }
        };
        if !matches!(result.token, Token::Comment(_)) && chr.is_whitespace() {
            result.continue_reading = false;
            return result;
        }
        match result.token {
            Token::StringLiteral(ref mut val) => {
                if chr == '\'' {
                    if *self.peek() == '\'' {
                        val.push('\'');
                        result.should_skip_next = true;
                    } else {
                        result.continue_reading = false;
                    }
                } else {
                    val.push(chr);
                }
            }
            Token::Comment(ref comment_type) => match comment_type {
                CommentType::Parenthesis => {
                    if chr == ')' && self.prev_char == Some('*') {
                        result.continue_reading = false;
                        return result;
                    }
                }
                CommentType::CurlyBrackets => {
                    if chr == '}' {
                        result.continue_reading = false;
                        return result;
                    }
                }
            },
            Token::Symbol(ref val) => {}
            Token::Operator(ref val) => {}
            Token::Keyword(ref val) => {}
            Token::IntLiteral(ref val) => {}
            _ => {}
        }
        result
    }
    pub fn read_token(&mut self) -> Token {
        self.reset_for_token_read();
        let mut read_result = self.read_first_char();
        if !read_result.continue_reading {
            return read_result.token;
        }
        self.is_first_char = false;
        while !self.is_eof() {
            if read_result.should_skip_next {
                self.read_next_char();
            } else {
                read_result = self.read_the_rest(read_result);
            }
        }
        return Token::Eof;
    }
}

pub fn get_tokens(source: &str) {
    let mut reader = Reader::new(source);
    while !reader.is_eof() {
        let token = reader.read_token();
        dbg!(token);
    }
}
