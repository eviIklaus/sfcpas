use multipeek::multipeek;
use common::Token;

pub fn parse_tokens(tokens_vec: &Vec<Token>) {
	let mut tokens = multipeek(tokens_vec);
	while tokens.peek().is_some() {
		let token = tokens.next();
	}
}
