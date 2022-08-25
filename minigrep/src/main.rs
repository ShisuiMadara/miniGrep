use std::env;
use std::fs;

struct Config{
    target_string: String,
    file_path: String
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = parse_config(&args);
    
}

fn parse_config(args: &[String]) -> Config{

    let target_string = args[1].clone();
    let file_path = args[2].clone();

    Config {target_string, file_path}
}
