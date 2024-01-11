use clap::{Parser, Subcommand};

mod board;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage your boards
    Board(board::Arguments),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Board(board) => match board.command {
            Some(board::Commands::Create(_args)) => {
                println!("Creating board...");
                todo!()
            }
            Some(board::Commands::View) => {
                println!("Viewing board...");
                todo!()
            }
            Some(board::Commands::Update) => {
                println!("Updating board...");
                todo!()
            }
            Some(board::Commands::Delete) => {
                println!("Deleting board...");
                todo!()
            }
            None => {
                println!("Unrecognized command");
            }
        },
    }
}
