use clap::{Parser, Subcommand};

use crate::core::{
    create_blank, extract_pages, inspect_pdf, merge_pdfs, remove_pages, reorder_pages,
    rotate_pages, set_metadata, split_pdf,
};

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
    /// PDF creation commands
    Create {
        #[command(subcommand)]
        command: CreateCommands,
    },
    /// Set metadata fields on a PDF
    SetMeta {
        input: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        author: Option<String>,
        #[arg(short, long)]
        output: String,
    },
    /// Reorder pages according to provided order list/ranges
    ReorderPages {
        input: String,
        #[arg(long)]
        order: String,
        #[arg(short, long)]
        output: String,
    },
    /// Split PDF into multiple parts
    Split {
        input: String,
        #[arg(long)]
        by: String,
        #[arg(long = "output-dir")]
        output_dir: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum CreateCommands {
    /// Create a blank single-page PDF
    Blank {
        #[arg(long)]
        size: String,
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
        Some(Commands::Create { command }) => match command {
            CreateCommands::Blank { size, output } => {
                create_blank(&size, std::path::Path::new(&output))?;
                println!("created=blank");
                println!("size={}", size);
                println!("output={}", output);
            }
        },
        Some(Commands::SetMeta {
            input,
            title,
            author,
            output,
        }) => {
            set_metadata(
                std::path::Path::new(&input),
                title.as_deref(),
                author.as_deref(),
                std::path::Path::new(&output),
            )?;
            println!("set_meta=true");
            println!("output={}", output);
        }
        Some(Commands::ReorderPages {
            input,
            order,
            output,
        }) => {
            reorder_pages(
                std::path::Path::new(&input),
                &order,
                std::path::Path::new(&output),
            )?;
            println!("reordered_pages={}", order);
            println!("output={}", output);
        }
        Some(Commands::Split {
            input,
            by,
            output_dir,
        }) => {
            let parts = split_pdf(
                std::path::Path::new(&input),
                &by,
                std::path::Path::new(&output_dir),
            )?;
            println!("split_by={}", by);
            println!("parts={}", parts);
            println!("output_dir={}", output_dir);
        }
        None => {}
    }
    Ok(())
}
