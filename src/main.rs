use clap::Parser;

//pub mod bean;

#[derive(Parser)]
struct Cli {
    root: String,
    command: String,
}

fn main() {
    let args = Cli::parse();

    println!("Root:     {}", args.root);
    println!("Command:  {}", args.command);
}