use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    
    let args:Vec<String> = env::args().collect();
    // dbg!(args);

    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let query = &args[1];
    // let path = &args[2];

    // println!("the string {}", config.query);
    // println!("the file path {}", config.path);

    // let content = fs::read_to_string(config.path).expect("should open the file");

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
    
    // println!("the content \n{content}");
    // println!("Current dir: {:?}", std::env::current_dir().unwrap());
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config{
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}


impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();

       let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
    // fn parse_config(args: &[String]) -> Config {
    
    //     let query = args[1].clone(); 
    //     let path = args[2].clone();
    
    //     Config {query, path}
    // }
}




// lessons learned from this mini cli app

// do we always use reference of referenced argument to get thire value - before it becomes struct it was a referenced value?
// but when we clone we didnt use the reference appersand
// i want to know about Result more!   what the hell is this ==> Result<Config, &'static str>?


// started with getting command line argument using env::args buildtin function
// convert it to collaction and get both the file path and the query.
// read the path using fs::read_to_string method and print the content.


// LEVEL 2 - MODULIRIZING THE CODEBASE

// started by implementing a function that take arguments(collection) and return the required value/prameters
// then instead of as value/tuple refactor the function to return a struct(one level upgrade, give a meaning for our cli arguments).

// instead of making a function that return a struct make a constructor that take argument and return a struct with everythign in place
// convert the return type to Result so we can catch errors properly of the argument is not given/not proper
// 

