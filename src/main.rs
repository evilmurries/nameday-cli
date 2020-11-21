use std::collections::HashMap;

fn parse_args() -> Option<Vec<String>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        return None;
    }
    Some(args)
}

fn compile_country_codes() -> HashMap<String, String> {
    let mut countries = HashMap::new();
    countries.insert("at".to_string(), "austria".to_string());
    countries.insert("bg".to_string(), "bulgaria".to_string());
    countries.insert("hr".to_string(), "croatia".to_string());
    countries.insert("cz".to_string(), "czechia".to_string());
    countries.insert("dk".to_string(), "denmark".to_string());
    countries.insert("ee".to_string(), "estonia".to_string());
    countries.insert("fi".to_string(), "finland".to_string());
    countries.insert("fr".to_string(), "france".to_string());
    countries.insert("de".to_string(), "germany".to_string());
    countries.insert("gr".to_string(), "greece".to_string());
    countries.insert("hu".to_string(), "hungary".to_string());
    countries.insert("it".to_string(), "italy".to_string());
    countries.insert("lv".to_string(), "latvia".to_string());
    countries.insert("lt".to_string(), "lithuania".to_string());
    countries.insert("pl".to_string(), "poland".to_string());
    countries.insert("ru".to_string(), "russia".to_string());
    countries.insert("sk".to_string(), "slovakia".to_string());
    countries.insert("es".to_string(), "spain".to_string());
    countries.insert("se".to_string(), "sweden".to_string());
    countries.insert("us".to_string(), "united states of america".to_string());
    countries
}

fn find_country(arg: &String, country_map: &HashMap<String, String>) -> Option<String> {
    let mut country_code: Option<String> = None;
    for (key, value) in country_map.iter() {
        if key == arg || value == arg {
            country_code = Some(key.to_string());
            break;
        }
    }
    country_code
}

fn main() {
    let mut args = match parse_args() {
        Some(args) => args,
        None => panic!("Supply the country code or name as argument."),
    };
    let countries = compile_country_codes();
    let country_code = match find_country(&args[1].to_lowercase(), &countries) {
        Some(code) => code,
        None => panic!("Incorrect country code provided"),
    };
    println!("{}", country_code);
}
