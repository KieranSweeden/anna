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
        Commands::Board(board_arguments) => match board_arguments.command {
            Some(board::Commands::Create(create_arguments)) => {
                println!("Creating board...");
                let board = board::Board::new(create_arguments.name, create_arguments.description);
                dbg!(board);
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
