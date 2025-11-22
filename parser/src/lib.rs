use multipeek::{multipeek, MultiPeek};
use common::Token;

pub fn parse_program(tokens_vec: &mut MultiPeek<std::slice::Iter<'_, Token>>) {
	let program_name = tokens_vec.next();
	let semicolon = tokens_vec.next();
	dbg!(program_name);
	dbg!(semicolon);
}

pub fn parse_tokens(tokens_vec: &Vec<Token>) {
	let mut tokens = multipeek(tokens_vec);
	while tokens.peek().is_some() {
		match tokens.next() {
			Some(Token::Keyword(val)) => match val.as_str() {
				"program" => parse_program(&mut tokens),
				_ => {}
			}
			None => break,
			_ => {},
		}
	}
}
