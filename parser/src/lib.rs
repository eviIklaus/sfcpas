use anyhow::bail;
use common::{Token, TokenType};
use multipeek::{MultiPeek, multipeek};
use std::collections::HashMap;

macro_rules! skip_comments {
    ($a:expr) => {
        loop {
            match $a.peek() {
                Some(token) => match &token.token_type {
                    TokenType::Comment(_) => {},
                    _ => break,
                },
                _ => break,
            }
            $a.next();
        }
    }
}

macro_rules! skip_expected {
    ($a:tt, $b:expr) => {
        skip_comments!($b);
        match $b.next() {
            Some(token) => match &token.token_type {
                TokenType::$a => {}
                _ => bail!("Expected {:?}, got: {:#?}", TokenType::$a, token),
            },
            None => bail!("Expected {:?}, got EOF.", TokenType::$a),
        };
    };
}

macro_rules! extract_expected {
    ($a:tt, $b:expr, $c:expr) => {
        {
            skip_comments!($c);
            match $c.next() {
                Some(token) => match &token.token_type {
                    TokenType::$a(val) => val,
                    _ => bail!("Expected {:?}, got: {:#?}", $b, token),
                },
                None => bail!("Expected {:?}, got EOF.", $b),
            }
        }
    };
}

pub fn parse_module_name(
    tokens: &mut MultiPeek<std::slice::Iter<'_, Token>>,
) -> anyhow::Result<()> {
    let module_name = extract_expected!(Identifier, "Module name", tokens);
    skip_expected!(Semicolon, tokens);
    println!("Module name: {}", module_name);

    Ok(())
}

pub fn parse_var(tokens: &mut MultiPeek<std::slice::Iter<'_, Token>>) -> anyhow::Result<()> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let mut current_var_names: Vec<String> = Vec::new();
    loop {
        skip_comments!(tokens);
        let var_name = match tokens.next() {
            Some(token) => match &token.token_type {
                TokenType::Identifier(val) => val,
                _ => break,
            },
            None => break,
        };
        current_var_names.push(var_name.to_string());
        match tokens.next() {
            Some(token) => match token.token_type {
                TokenType::Comma => continue,
                TokenType::Colon => {
                    let var_type =
                        extract_expected!(Identifier, "variable type", tokens).to_string();
                    skip_expected!(Semicolon, tokens);
                    for var_name in &current_var_names {
                        vars.insert(var_name.clone(), var_type.clone());
                    }
                    current_var_names.clear();
                }
                _ => bail!("Expected comma or colon, got: {:#?}", token),
            },
            None => bail!("Expected comma or colon, got EOF"),
        }
    }
    println!("New variables: {:#?}", vars);
    Ok(())
}

pub fn parse_program_stmt() {

}

pub fn parse_tokens(tokens_vec: &Vec<Token>) -> anyhow::Result<()> {
    let mut tokens = multipeek(tokens_vec);
    while tokens.peek().is_some() {
        let token = tokens.next();
        if token.is_none() {
            break;
        }
        let token =
            token.expect("Impossible! Token shouldn't be none here, it's checked before this.");
        let token_type = &token.token_type;
        match token_type {
            TokenType::Keyword(val) => match val.as_str() {
                "program" => parse_module_name(&mut tokens)?,
                "unit" => parse_module_name(&mut tokens)?,
                "var" => parse_var(&mut tokens)?,
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
