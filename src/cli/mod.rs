use clap::{Parser, Subcommand};

use crate::core::inspect_pdf;

#[derive(Debug, Parser)]
#[command(name = "pdf-toolkit")]
#[command(about = "Spec-driven PDF toolkit in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inspect a PDF file
    Info { input: String },
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Info { input }) => {
            let info = inspect_pdf(std::path::Path::new(&input))?;
            println!("version={}", info.version);
            println!("pages={}", info.page_count);
            println!("encrypted={}", info.encrypted);
            println!("title={}", info.title.as_deref().unwrap_or(""));
            println!("author={}", info.author.as_deref().unwrap_or(""));
        }
        None => {}
    }
    Ok(())
}
