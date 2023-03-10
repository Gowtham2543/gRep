use std::{env, process};
use gRep::{Config, run};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.path);

    if let Err(e) = run(config) {
        eprintln!("Application Error : {e}");
        process::exit(1);
    }
}




