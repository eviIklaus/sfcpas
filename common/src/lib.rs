#[derive(Debug)]
pub enum CommentType {
    Slash,
    Parenthesis,
    CurlyBrackets,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    IntLiteral(String),
    FloatLiteral(String),
    CharLiteral(String),
    StringLiteral(String),
    Operator(String),
    Symbol(String),
    Semicolon,
    Begin,
    End,
    StatementEnd,
    ProgramEnd,
    Comment(CommentType),
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    Eof,
    Null,
}

pub static OPERATORS: &'static [char] = &[
    '+', '-', '*', '/', // Arithmetic operators
    '=', '<', '>', // Relational operators
];

pub static SYMBOLS: &'static [char] =
    &[';', ':', ',', '.', '[', ']', '(', ')', '^', '@', '\'', '$'];
