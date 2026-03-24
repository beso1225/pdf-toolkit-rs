use clap::{Parser, Subcommand};

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
    let _cli = Cli::parse();
    Ok(())
}
