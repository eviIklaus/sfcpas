use common::{CommentType, TokenType, Token};
use multipeek::{MultiPeek, multipeek};
use std::str::Chars;

#[derive(Debug)]
struct ReadTokenResult {
    should_skip_next: bool,
    continue_reading: bool,
    token: Token,
}

impl ReadTokenResult {
    pub fn new(col: usize, line: usize) -> Self {
        Self {
            should_skip_next: false,
            continue_reading: true,
            token: Token {
                token_type: TokenType::Eof,
                col, line
            }
        }
    }
}

#[derive(Debug)]
struct Reader<'a> {
    source_iter: MultiPeek<Chars<'a>>,
    prev_char: Option<char>,
    current_char: Option<char>,

    is_first_char: bool,
    current_col: usize,
    current_line: usize,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            is_first_char: true,
            prev_char: None,
            current_char: None,
            source_iter: multipeek(source.chars()),

            current_col: 1,
            current_line: 1,
        }
    }
    pub fn reset_for_token_read(&mut self) {
        self.is_first_char = true;
    }
    pub fn is_eof(&mut self) -> bool {
        matches!(self.source_iter.peek(), None)
    }
    fn peek(&mut self) -> &char {
        self.source_iter.peek().unwrap_or(&'\0')
    }
    fn peek_2(&mut self) -> &char {
        self.source_iter.peek_nth(1).unwrap_or(&'\0')
    }
    fn skip_whitespace(&mut self) {
        while self.source_iter.peek().unwrap_or(&'\0').is_whitespace() {
            self.read_next_char();
        }
    }
    fn read_next_char(&mut self) -> Option<char> {
        self.prev_char = self.current_char;
        self.current_char = self.source_iter.next();
        match self.current_char {
            Some('\n') => {
                self.current_col = 1;
                self.current_line += 1;
            },
            Some(_) => {
                self.current_col += 1;
            },
            None => {}
        }
        self.current_char
    }
    fn read_first_char(&mut self) -> ReadTokenResult {
        self.skip_whitespace();
        let mut result = ReadTokenResult::new(self.current_col, self.current_line);
        match self.read_next_char() {
            None => result.continue_reading = false,
            Some(chr) => {
                // Check if it's an assignment operator
                if chr == ':' && *self.peek() == '=' {
                    result.continue_reading = false;
                    result.should_skip_next = true;
                    result.token.token_type = TokenType::AssignmentOperator;
                    return result;
                }
                let mut is_single_char = true;
                result.token.token_type = match chr {
                    '{' => {
                        self.is_first_char = false;
                        result.token.token_type = TokenType::Comment(CommentType::CurlyBrackets);
                        return result;
                    }
                    '[' => TokenType::OpenSquareBracket,
                    ']' => TokenType::CloseSquareBracket,
                    '^' => TokenType::PointerSymbol,
                    ';' => TokenType::Semicolon,
                    '$' => TokenType::DollarSign,
                    '&' => TokenType::Ampersand,
                    '@' => TokenType::AtSign,
                    ')' => TokenType::CloseParenthesis,
                    '#' => TokenType::Hash,
                    ':' => TokenType::Colon,
                    '.' => TokenType::Period,
                    ',' => TokenType::Comma,
                    _ => {
                        is_single_char = false;
                        result.token.token_type
                    }
                };
                if is_single_char {
                    result.continue_reading = false;
                    return result;
                }
                // Check if it's either a beginning of a comment or just a parenthesis.
                if chr == '(' {
                    if *self.peek() == '*' {
                        result.token.token_type = TokenType::Comment(CommentType::Parenthesis)
                    } else {
                        result.continue_reading = false;
                        result.token.token_type = TokenType::OpenParenthesis
                    }
                } else if chr == '\'' {
                    // Check if a single quote is in the string literal
                    result.token.token_type = TokenType::StringLiteral(String::new());
                    if *self.peek() == '\'' {
                        // Check if the string literal is just empty
                        if *self.peek_2() != '\'' {
                            result.should_skip_next = true;
                            result.continue_reading = false;
                        }
                    }
                } else if chr.is_alphabetic() || chr == '_' {
                    // Check if it's a possible identifier.
                    result.token.token_type = TokenType::Identifier(chr.to_string());
                    let next = *self.peek();
                    result.continue_reading = next == '_' || next.is_alphanumeric();
                } else if chr.is_ascii_digit() {
                    // Check if it's a possible integer literal.
                    result.token.token_type = TokenType::IntLiteral(chr.to_string());
                    result.continue_reading = self.peek().is_ascii_digit();
                } else if common::OPERATORS.contains(&chr) {
                    // Check if it's a possible operator.
                    result.token.token_type = TokenType::Operator(chr.to_string());
                    let next = *self.peek();
                    // Check if it's an equality operator or if the operator ends here.
                    if chr == '=' || !common::OPERATORS.contains(&next) {
                        result.continue_reading = false;
                    }
                }
            }
        }
        result
    }
    fn read_the_rest(&mut self, mut result: ReadTokenResult) -> ReadTokenResult {
        let chr = match self.read_next_char() {
            Some(chr) => chr,
            None => {
                result.continue_reading = false;
                return result;
            }
        };
        if !matches!(result.token.token_type, TokenType::Comment(_))
            && !matches!(result.token.token_type, TokenType::StringLiteral(_))
            && chr.is_whitespace()
        {
            result.continue_reading = false;
            return result;
        }
        match result.token.token_type {
            TokenType::StringLiteral(ref mut val) => {
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
            TokenType::Comment(ref comment_type) => match comment_type {
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
            TokenType::Operator(ref mut val) => {
                val.push(chr);
                // Check if it's an assignment operator or
                // if it reached the end of an operator in the next character.
                let next = *self.peek();
                if chr == '=' || !common::OPERATORS.contains(&next) {
                    result.continue_reading = false;
                }
            }
            TokenType::Identifier(ref mut val) => {
                val.push(chr);
                let next = *self.peek();
                if !next.is_alphanumeric() && next != '_' {
                    result.continue_reading = false;
                }
            }
            TokenType::IntLiteral(ref mut val) => {
                val.push(chr);
                let next = *self.peek();
                if !next.is_ascii_digit() {
                    result.continue_reading = false;
                }
            }
            _ => {}
        }
        result
    }
    pub fn read_token(&mut self) -> Token {
        self.reset_for_token_read();
        let mut result = self.read_first_char();
        if result.should_skip_next {
            self.read_next_char();
        }
        if !result.continue_reading {
            return result.token;
        }
        self.is_first_char = false;
        while !self.is_eof() {
            if !result.continue_reading {
                return result.token;
            }
            if result.should_skip_next {
                self.read_next_char();
                result.should_skip_next = false;
            } else {
                result = self.read_the_rest(result);
            }
        }
        result.token
    }
}

pub fn get_tokens(source: &str) -> Vec<Token> {
    let mut reader = Reader::new(source);
    let mut tokens = Vec::new();
    while !reader.is_eof() {
        let mut token = reader.read_token();
        if let TokenType::Identifier(ref val) = token.token_type {
            if common::RESERVED_WORDS.contains(&val.to_lowercase().as_str()) {
                token.token_type = TokenType::Keyword(val.to_lowercase());
            } else {
                token.token_type = TokenType::Identifier(val.to_lowercase());
            }
        }
        match token.token_type {
            TokenType::Comment(_) => { /* Skip comments. */ },
            _ => tokens.push(token),
        }
    }
    tokens
}
