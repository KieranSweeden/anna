use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage your boards
    Board(BoardArgs),
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
struct BoardArgs {
    #[command(subcommand)]
    command: Option<BoardCommands>,
}

#[derive(Debug, Subcommand)]
enum BoardCommands {
    Create,
    View,
    Update,
    Delete,
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Board(command)) => match command.command {
            Some(BoardCommands::Create) => {
                println!("Creating board...");
                todo!()
            }
            Some(BoardCommands::View) => {
                println!("Viewing board...");
                todo!()
            }
            Some(BoardCommands::Update) => {
                println!("Updating board...");
                todo!()
            }
            Some(BoardCommands::Delete) => {
                println!("Deleting board...");
                todo!()
            }
            None => {
                println!("Unrecognized command");
            }
        },
        None => {}
    }

    // Continued program logic goes here...
}
