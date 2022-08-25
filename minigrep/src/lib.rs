use std::error::Error;
use std::fs;


pub struct Config{
    target_string: String,
    file_path: String
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

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.target_string, &contents){
        println!("{line}");
    }
        
    //println!("{contents}");

    Ok(())
}

pub fn search<'a>(target_string:&'a str, text: &'a str) -> Vec<&'a str>{
    
    let mut results = Vec::new();

    for line in text.lines(){
        if line.contains(target_string){
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests{
    
    use super::*;

    #[test]
    fn test1(){
        let query = "duct";
        let contents = "safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}