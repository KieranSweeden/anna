use clap::{Args, Subcommand};

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(CreateArguments),
    View,
    Update,
    Delete,
}

#[derive(Debug, Args)]
pub struct CreateArguments {
    /// The name of the board
    #[arg(short, long)]
    name: Option<String>,

    /// A description of the board
    #[arg(short, long)]
    description: Option<String>,
}
