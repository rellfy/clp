use std::env;
use clp::CommandLineParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let clp = CommandLineParser::new(args);

    println!("Has key \"-X\"? {}", (clp.has_key("-X")));

    let y_value = clp.key_value("--y").unwrap_or("None".to_owned());
    println!("Key \"--y\" value: {}", y_value);
}
