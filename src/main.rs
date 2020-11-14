
use std::collections::HashMap;
use std::error::Error;

fn parse_args() -> Result<Vec<String> , Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    Ok(args)
}

fn compile_country_codes() -> HashMap<String, String> {
    let mut countries = HashMap::new();
    countries.insert("at".to_string(), "Austria".to_string());
    countries.insert("bg".to_string(), "Bulgaria".to_string());
    countries.insert("hr".to_string(), "Croatia".to_string());
    countries.insert("cz".to_string(), "Czechia".to_string());
    countries.insert("dk".to_string(), "Denmark".to_string());
    countries.insert("ee".to_string(), "Estonia".to_string());
    countries.insert("fi".to_string(), "Finland".to_string());
    countries.insert("fr".to_string(), "France".to_string());
    countries.insert("de".to_string(), "Germany".to_string());
    countries.insert("gr".to_string(), "Greece".to_string());
    countries.insert("hu".to_string(), "Hungary".to_string());
    countries.insert("it".to_string(), "Italy".to_string());
    countries.insert("lv".to_string(), "Latvia".to_string());
    countries.insert("lt".to_string(), "Lithuania".to_string());
    countries.insert("pl".to_string(), "Poland".to_string());
    countries.insert("ru".to_string(), "Russia".to_string());
    countries.insert("sk".to_string(), "Slovakia".to_string());
    countries.insert("es".to_string(), "Spain".to_string());
    countries.insert("se".to_string(), "Sweden".to_string());
    countries.insert("us".to_string(), "United States of America".to_string());
    countries
}

fn main() {
    match parse_args() {
        Ok(args) => println!("{:?}", args),
        Err(error)=> panic!("Something broke: {}", error),
    };
    let countries = compile_country_codes();
    println!("{}", countries.get("pl").unwrap())
}
