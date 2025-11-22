#[derive(Debug)]
pub enum CommentType {
    Parenthesis,
    CurlyBrackets,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    IntLiteral(String),
    FloatLiteral(String),
    CharLiteral(char),
    StringLiteral(String),
    AssignmentOperator,
    Operator(String),
    Symbol(String),
    Begin,
    End,
    StatementEnd,
    ProgramEnd,
    Comment(CommentType),
    Hash,
    Colon,
    Semicolon,
    PointerSymbol,
    DollarSign,
    AtSign,
    Ampersand,
    OpenParenthesis,
    CloseParenthesis,
    OpenSquareBracket,
    CloseSquareBracket,
    Period,
    Comma,
    Eof,
    Null,
}

pub static OPERATORS: &'static [char] = &[
    '+', '-', '*', '/', // Arithmetic operators
    '=', '<', '>', // Relational operators
];

// Based on: https://wiki.freepascal.org/The_parser#Module_information
#[derive(Debug)]
pub struct Module {
    is_unit: bool,
    name: String,

    used_units: Vec<String>,
}