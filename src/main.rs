use clap::{Parser, Subcommand};
use dotenv::dotenv;
use libsql_client::Client;

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

async fn get_db_client() -> Client {
    let db_url = dotenv::var("DB_URL").expect("Failed to retrieve 'DB_URL' environment variable");

    let db_auth_token = dotenv::var("DB_AUTH_TOKEN")
        .expect("Failed to retrieve 'DB_AUTH_TOKEN' environment variable");

    return libsql_client::Client::from_config(libsql_client::Config {
        url: url::Url::parse(&db_url).unwrap(),
        auth_token: Some(db_auth_token),
    })
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_client = get_db_client().await;

    let example_data_str = include_str!("example-data.sql");

    let result_set = db_client.execute(example_data_str).await;

    dbg!(result_set);

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
