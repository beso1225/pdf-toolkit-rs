use clap::{Parser, Subcommand};

use crate::core::{extract_pages, inspect_pdf, merge_pdfs, remove_pages, rotate_pages};

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
    /// Extract a page subset into a new PDF
    ExtractPages {
        input: String,
        #[arg(long)]
        pages: String,
        #[arg(short, long)]
        output: String,
    },
    /// Remove selected pages from a PDF
    RemovePages {
        input: String,
        #[arg(long)]
        pages: String,
        #[arg(short, long)]
        output: String,
    },
    /// Rotate selected pages in a PDF
    RotatePages {
        input: String,
        #[arg(long)]
        pages: String,
        #[arg(long)]
        deg: i32,
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
        Some(Commands::ExtractPages {
            input,
            pages,
            output,
        }) => {
            extract_pages(
                std::path::Path::new(&input),
                &pages,
                std::path::Path::new(&output),
            )?;
            println!("extracted_pages={}", pages);
            println!("output={}", output);
        }
        Some(Commands::RemovePages {
            input,
            pages,
            output,
        }) => {
            remove_pages(
                std::path::Path::new(&input),
                &pages,
                std::path::Path::new(&output),
            )?;
            println!("removed_pages={}", pages);
            println!("output={}", output);
        }
        Some(Commands::RotatePages {
            input,
            pages,
            deg,
            output,
        }) => {
            rotate_pages(
                std::path::Path::new(&input),
                &pages,
                deg,
                std::path::Path::new(&output),
            )?;
            println!("rotated_pages={}", pages);
            println!("degrees={}", deg);
            println!("output={}", output);
        }
        None => {}
    }
    Ok(())
}
