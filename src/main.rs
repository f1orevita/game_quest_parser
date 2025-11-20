use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};
use game_quest_parser_Hodik::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "game_quest_parser")]
#[command(about = "A CLI tool to parse Game Quest Definition Language files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        file: PathBuf,
    },
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Credits => {
            println!("Game Quest Parser v0.1.0");
            println!("Created by: f1ore vita");
            println!("Theme: Custom Language for RPG Quests");
        }
        Commands::Parse { file } => {
            println!("Reading file: {:?}", file);
            let content = fs::read_to_string(&file)
                .with_context(|| format!("Failed to read file {:?}", file))?;

            println!("Parsing content...");
            let mut parser = Parser::new(&content).context("Failed to initialize parser")?;

            let quest = parser
                .parse_quest()
                .context("Failed to parse quest syntax")?;

            println!("✅ Successfully parsed!");
            println!("{:#?}", quest);
        }
    }

    Ok(())
}
