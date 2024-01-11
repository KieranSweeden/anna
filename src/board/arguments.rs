use clap::Args;

#[derive(Debug, Args)]
pub struct Create {
    /// The name of the board
    #[arg(short, long)]
    name: Option<String>,

    /// A description of the board
    #[arg(short, long)]
    description: Option<String>,
}
