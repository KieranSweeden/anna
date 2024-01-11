use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    Create(CreateBoardArgs),
    View,
    Update,
    Delete,
}

#[derive(Debug, Args)]
struct CreateBoardArgs {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    description: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Board(board) => match board.command {
            Some(BoardCommands::Create(_args)) => {
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
    }
}
