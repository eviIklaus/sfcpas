use anyhow::bail;
use multipeek::{multipeek, MultiPeek};
use common::{Token, TokenType};

pub fn parse_module_name(tokens_vec: &mut MultiPeek<std::slice::Iter<'_, Token>>) -> anyhow::Result<()> {
	let program_name = match tokens_vec.next() {
		Some(token) => match &token.token_type {
			TokenType::Identifier(val) => {
				val
			},
			_ => bail!("Expected module name, got unknown token type: {:?}", token),
		},
		None => bail!("Expected module name, got EOF."),
	};
	match tokens_vec.next() {
		Some(token) => match &token.token_type {
			TokenType::Semicolon => {},
			_ => bail!("Expected semicolon, got unknown token type: {:?}", token),
		},
		None => bail!("Expected semicolon, got EOF."),
	};
	println!("Program name: {}", program_name);

	Ok(())
}

pub fn parse_tokens(tokens_vec: &Vec<Token>) -> anyhow::Result<()> {
	let mut tokens = multipeek(tokens_vec);
	while tokens.peek().is_some() {
		let token = tokens.next();
		if token.is_none() {
			break;
		}
		let token = token.expect("Impossible! Token shouldn't be none here, it's checked before this.");
		let token_type = &token.token_type;
		match token_type {
			TokenType::Keyword(val) => match val.as_str() {
				"program" => parse_module_name(&mut tokens)?,
				"unit" => parse_module_name(&mut tokens)?,
				_ => {}
			}
			_ => {},
		}
	}
	Ok(())
}
