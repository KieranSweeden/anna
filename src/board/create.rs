use clap::Args;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct Arguments {
    /// The name of the board
    #[arg(short, long)]
    pub name: String,

    /// A description of the board
    #[arg(short, long)]
    pub description: Option<String>,
}
