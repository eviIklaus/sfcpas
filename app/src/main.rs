use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No source files.");
        return;
    }

    for arg in &args[1..] {
        println!("{:?}", arg);
    }
}
