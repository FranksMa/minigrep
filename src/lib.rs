use std::error::Error;
use std::fs;



struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config {query, filename})
    
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error> > {

}