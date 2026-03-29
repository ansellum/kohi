use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::Path;
use std::ffi::OsStr;

pub mod equipment;
pub mod bag;
pub mod coffee;
pub mod brew;

#[derive(Parser)]
#[command(version, about)]
#[command(next_line_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Modes,
}

#[derive(Subcommand)]
enum Modes {
    Equipment(EquipmentArgs),
    Bag(BagArgs),
    Coffee(CoffeeArgs),
    Brew(BrewArgs),
    Import {
        file: String,
    }
}

/////////////////////
// WIZARD COMMANDS //
/////////////////////
#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct EquipmentArgs {
    #[command(subcommand)]
    command: EquipmentCommands,
}

#[derive(Subcommand)]
enum EquipmentCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct BagArgs {
    #[command(subcommand)]
    command: BagCommands,
}

#[derive(Subcommand)]
enum BagCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct CoffeeArgs {
    #[command(subcommand)]
    command: CoffeeCommands,
}

#[derive(Subcommand)]
enum CoffeeCommands {
    Add,
    Remove,
    List,
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct BrewArgs {
    #[command(subcommand)]
    command: BrewCommands,
}

#[derive(Subcommand)]
enum BrewCommands {
    Add,
    Remove,
    List,
}


/////////////////////
// IMPORT COMMANDS //
/////////////////////

fn get_extension_from_filename(filename: &str) -> Option<&str>  {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialization
    let args = Cli::parse();

    match args.command {
        // WIZARD
        Modes::Equipment(equipment) => {
            match equipment.command {
                EquipmentCommands::Add => println!("kaffe equipment add"),
                EquipmentCommands::Remove => println!("kaffe equipment remove"),
                EquipmentCommands::List => println!("kaffe equipment list"),
            }
        }
        Modes::Bag(bag) => {
            match bag.command {
                BagCommands::Add => println!("kaffe bag add"),
                BagCommands::Remove => println!("kaffe bag remove"),
                BagCommands::List => println!("kaffe bag list"),
            }
        }
        Modes::Coffee(coffee) => {
            match coffee.command {
                CoffeeCommands::Add => println!("kaffe coffee add"),
                CoffeeCommands::Remove => println!("kaffe coffee remove"),
                CoffeeCommands::List => println!("kaffe coffee list"),
            }
        }
        Modes::Brew(brew) => {
            match brew.command {
                BrewCommands::Add => println!("kaffe brew add"),
                BrewCommands::Remove => println!("kaffe brew remove"),
                BrewCommands::List => println!("kaffe brew list"),
            }
        }

        // MAIN
        Modes::Import { file } => {
            // match get_extension_from_filename(&path) {
            //     None => panic!("Invalid file type. File must be either CSV or JSON"),
            //     Some(extension) => {
            //         match extension {
            //             _ => panic!("Invalid file type. File must be either CSV or JSON"),
            //             "json" => println!("json"),
            //             "csv" => println!("json"),
            //         }
            //     }
            // }

            let extension = get_extension_from_filename(&file)
                .expect("Invalid file type. Please use a JSON or CSV.");
            
            match extension {
                "json" => println!("json"),
                "csv" => println!("csv"),
                _ => panic!("invalid type")
            }
         
        }
    }

    Ok(())
}

