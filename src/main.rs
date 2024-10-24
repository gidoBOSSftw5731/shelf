mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::{get_config_dir, get_config_path, load_config};

#[derive(Parser)]
#[command(
    version,
    about = "Shelf - Your personal command-line bookshelf for storing and recalling useful commands",
    long_about = "
A lightweight command-line bookshelf for storing and recalling useful commands. No need to dig 
through shell history for that complex Docker command or git operation - just shelf it and find 
it when you need it. (as long as you shelved it of course)

No more \"I know I used this command last month, but what was it again?\" moments. Just shelf 
it, and find it when you need it."
)]
struct ShelfCli {
    // /// Optional name to operate on
    // name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Test {
    //     /// lists test values
    //     #[arg(short = "short list", long)]
    //     list: bool,
    // },
    Config,
    // TODO: Save command
    // TODO: List command
}

fn main() -> Result<()> {
    let config_dir = get_config_dir();
    let config_path = get_config_path(&config_dir);

    let config = load_config(&config_dir, &config_path).context("Could not load config!")?;
    let cli = ShelfCli::parse();

    match &cli.command {
        Some(Commands::Config) => {
            println!("{:?} is the config dir", config_dir);
            println!("{:?} is the config path", config_path);
            println!("{:?} is the storage path", config.storage_path);
        }
        None => {}
    }

    Ok(())
}
