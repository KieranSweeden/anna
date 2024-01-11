use clap::{Args, Subcommand};

mod create;

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(create::Arguments),
    View,
    Update,
    Delete,
}
