use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename})
    
    }
}

/// # 介绍
/// 程序的运行入口

pub fn run(config: Config) -> Result<(),Box<dyn Error> > {
    let contents = fs::read_to_string(config.filename)?;
    println!("with text :\n{ } \n find { }",contents, config.query);
    Ok(())
}