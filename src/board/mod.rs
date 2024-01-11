use clap::{Args, Subcommand};
use uuid::Uuid;

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

#[derive(Debug)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

impl Board {
    pub fn new(name: String, description: Option<String>) -> Self {
        Board {
            id: Uuid::new_v4(),
            name,
            description,
        }
    }
}
