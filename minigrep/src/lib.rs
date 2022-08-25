use std::error::Error;
use std::fs;


pub struct Config{
    target_string: String,
    file_path: String
}


pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;
        
    println!("{contents}");

    Ok(())
}


impl Config{

    pub fn parse_config(args: &[String]) -> Result<Config,&str>{

        if args.len()<3 {
            return Err("Not enough arguments");
        }

        let target_string = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config {target_string, file_path})
    }

}