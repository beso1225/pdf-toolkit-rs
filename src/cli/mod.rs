use clap::{Parser, Subcommand};

use crate::core::{inspect_pdf, merge_pdfs};

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
    /// Merge PDF files in order
    Merge {
        inputs: Vec<String>,
        #[arg(short, long)]
        output: String,
    },
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
        Some(Commands::Merge { inputs, output }) => {
            let refs: Vec<&std::path::Path> = inputs.iter().map(std::path::Path::new).collect();
            merge_pdfs(&refs, std::path::Path::new(&output))?;
            println!("merged_pages_source_count={}", refs.len());
            println!("output={}", output);
        }
        None => {}
    }
    Ok(())
}
