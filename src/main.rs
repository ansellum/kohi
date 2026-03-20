use std::env;
use std::process;

//pub mod bean;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Root command: {}", config.root);
    println!("Subcommand: {}", config.command);
}

struct Config {
    root: String,
    command: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let root = args[1].clone();
        let command = args[2].clone();

        Ok(Config { root, command })
    }
}