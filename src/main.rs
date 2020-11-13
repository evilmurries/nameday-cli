
use std::error::Error;

fn parse_args() -> Result<Vec<String> , Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    Ok(args)
}

fn main() {
    match parse_args() {
        Ok(args) => println!("{:?}", args),
        Err(error)=> panic!("Something broke: {}", error),
    };
}
