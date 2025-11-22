use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No source files.");
        return Ok(());
    }

    for arg in &args[1..] {
        let source: String = fs::read_to_string(arg)?;
        lexer::get_tokens(&source);
    }

    Ok(())
}
