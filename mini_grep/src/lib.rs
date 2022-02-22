use std::{env, error::Error, fs};

const IS_CASE_SENSITIVE_KEY: &str = "IS_CASE_SENSITIVE_KEY";

#[derive(Debug, PartialEq)]
pub struct Config{
    filename: String,
    keyword: String,
    is_case_sensitive: bool
}

impl Config{

    fn new(filename: String, keyword: String, is_case_sensitive: bool) -> Config{
        Config { filename, keyword, is_case_sensitive }
    }
}



pub fn get_config_data(args: &[String]) -> Result<Config, &str> {
    if args.len() != 3 {
        return Err("one or more arguments are missing arguments format is keyword filepath");
    }
    let is_set = env::var(IS_CASE_SENSITIVE_KEY).is_ok();

    let filename = args[2].clone();
    let keyword = args[1].clone();

  Ok(Config::new(filename, keyword, is_set))
}

pub fn search_text(config: Config) -> Result<(), Box<dyn Error>>{

    let raw_text = fs::read_to_string(config.filename)?;
    let mut matches: Vec<(usize, &str)> = vec![];

    let query: String = if config.is_case_sensitive { config.keyword } else { config.keyword.to_lowercase() };

    let text_lines: Vec<String>  = if !config.is_case_sensitive{
        raw_text.lines().into_iter().into_iter().map(|line| {
            line.to_lowercase()
        }).collect::<Vec<String>>()
    }else{
        raw_text.lines().map(|line| {line.to_string()}).collect::<Vec<String>>()
    };

    for (line_no,line) in text_lines.iter().enumerate(){

        if line.contains(&query){
            matches.push((line_no, line));
        }
    }
    for (line_no,line) in matches{
        println!("{}, {}", line_no, line)
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::get_config_data;

    #[test]
    pub fn incomplete_data(){
        assert_eq!(get_config_data(&[]), Err("one or more arguments are missing arguments format is keyword filepath"))
    }
}
