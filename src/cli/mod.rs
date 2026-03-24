use clap::{Parser, Subcommand, ValueEnum};
use serde_json::json;
use std::io::{self, Write};

use crate::core::{
    create_blank, extract_pages, inspect_pdf, merge_pdfs_with_options, remove_pages, reorder_pages,
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
    Info {
        input: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Merge PDF files in order
    #[command(
        after_help = "Examples:\n  pdf merge a.pdf b.pdf -o merged.pdf\n  pdf merge a.pdf b.pdf --index -o merged-index.pdf\n  pdf merge a.pdf b.pdf --index --links=false --outlines=false -o merged-min.pdf\n\nNote: --links/--outlines are only effective when --index is enabled."
    )]
    Merge {
        inputs: Vec<String>,
        #[arg(long, default_value_t = false)]
        index: bool,
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        links: bool,
        #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
        outlines: bool,
        #[arg(short, long)]
        output: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Extract a page subset into a new PDF
    ExtractPages {
        input: String,
        #[arg(long)]
        pages: String,
        #[arg(short, long)]
        output: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Remove selected pages from a PDF
    RemovePages {
        input: String,
        #[arg(long)]
        pages: String,
        #[arg(short, long)]
        output: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Reorder pages according to provided order list/ranges
    ReorderPages {
        input: String,
        #[arg(long)]
        order: String,
        #[arg(short, long)]
        output: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Split PDF into multiple parts
    #[command(
        after_help = "Examples:\n  pdf split input.pdf --by single --output-dir parts\n  pdf split input.pdf --by range:1-2,4-5 --output-dir parts\n  pdf split input.pdf --by chunk:3 --output-dir parts"
    )]
    Split {
        input: String,
        #[arg(long)]
        by: String,
        #[arg(long = "output-dir")]
        output_dir: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
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
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.command.is_none() {
        return run_interactive_shell();
    }
    execute_command(cli)
}

fn execute_command(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::Info { input, format }) => {
            let info = inspect_pdf(std::path::Path::new(&input))?;
            match format {
                OutputFormat::Text => {
                    print_ok("info");
                    println!("version={}", info.version);
                    println!("pages={}", info.page_count);
                    println!("encrypted={}", info.encrypted);
                    println!("title={}", info.title.as_deref().unwrap_or(""));
                    println!("author={}", info.author.as_deref().unwrap_or(""));
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "info",
                        "version": info.version,
                        "pages": info.page_count,
                        "encrypted": info.encrypted,
                        "title": info.title.unwrap_or_default(),
                        "author": info.author.unwrap_or_default(),
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::Merge {
            inputs,
            index,
            links,
            outlines,
            output,
            format,
        }) => {
            let refs: Vec<&std::path::Path> = inputs.iter().map(std::path::Path::new).collect();
            let effective_links = if index { links } else { false };
            let effective_outlines = if index { outlines } else { false };
            merge_pdfs_with_options(
                &refs,
                std::path::Path::new(&output),
                index,
                effective_links,
                effective_outlines,
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("merge");
                    println!("merged_pages_source_count={}", refs.len());
                    println!("index={}", index);
                    println!("links={}", effective_links);
                    println!("outlines={}", effective_outlines);
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "merge",
                        "merged_pages_source_count": refs.len(),
                        "index": index,
                        "links": effective_links,
                        "outlines": effective_outlines,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::ExtractPages {
            input,
            pages,
            output,
            format,
        }) => {
            extract_pages(
                std::path::Path::new(&input),
                &pages,
                std::path::Path::new(&output),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("extract-pages");
                    println!("extracted_pages={}", pages);
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "extract-pages",
                        "extracted_pages": pages,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::RemovePages {
            input,
            pages,
            output,
            format,
        }) => {
            remove_pages(
                std::path::Path::new(&input),
                &pages,
                std::path::Path::new(&output),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("remove-pages");
                    println!("removed_pages={}", pages);
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "remove-pages",
                        "removed_pages": pages,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::RotatePages {
            input,
            pages,
            deg,
            output,
            format,
        }) => {
            rotate_pages(
                std::path::Path::new(&input),
                &pages,
                deg,
                std::path::Path::new(&output),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("rotate-pages");
                    println!("rotated_pages={}", pages);
                    println!("degrees={}", deg);
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "rotate-pages",
                        "rotated_pages": pages,
                        "degrees": deg,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::Create { command }) => match command {
            CreateCommands::Blank {
                size,
                output,
                format,
            } => {
                create_blank(&size, std::path::Path::new(&output))?;
                match format {
                    OutputFormat::Text => {
                        print_ok("create-blank");
                        println!("created=blank");
                        println!("size={}", size);
                        println!("output={}", output);
                    }
                    OutputFormat::Json => {
                        let payload = json!({
                            "status": "ok",
                            "command": "create-blank",
                            "created": "blank",
                            "size": size,
                            "output": output,
                        });
                        println!("{}", payload);
                    }
                }
            }
        },
        Some(Commands::SetMeta {
            input,
            title,
            author,
            output,
            format,
        }) => {
            set_metadata(
                std::path::Path::new(&input),
                title.as_deref(),
                author.as_deref(),
                std::path::Path::new(&output),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("set-meta");
                    println!("set_meta=true");
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "set-meta",
                        "set_meta": true,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::ReorderPages {
            input,
            order,
            output,
            format,
        }) => {
            reorder_pages(
                std::path::Path::new(&input),
                &order,
                std::path::Path::new(&output),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("reorder-pages");
                    println!("reordered_pages={}", order);
                    println!("output={}", output);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "reorder-pages",
                        "reordered_pages": order,
                        "output": output,
                    });
                    println!("{}", payload);
                }
            }
        }
        Some(Commands::Split {
            input,
            by,
            output_dir,
            format,
        }) => {
            let parts = split_pdf(
                std::path::Path::new(&input),
                &by,
                std::path::Path::new(&output_dir),
            )?;
            match format {
                OutputFormat::Text => {
                    print_ok("split");
                    println!("split_by={}", by);
                    println!("parts={}", parts);
                    println!("output_dir={}", output_dir);
                }
                OutputFormat::Json => {
                    let payload = json!({
                        "status": "ok",
                        "command": "split",
                        "split_by": by,
                        "parts": parts,
                        "output_dir": output_dir,
                    });
                    println!("{}", payload);
                }
            }
        }
        None => {}
    }
    Ok(())
}

fn print_ok(command: &str) {
    println!("status=ok");
    println!("command={command}");
}

fn run_interactive_shell() -> anyhow::Result<()> {
    print_shell_banner();
    println!("Type `help` for shell commands, or `quit` to exit.");
    println!("Try: info <file.pdf>  |  merge a.pdf b.pdf -o out.pdf");

    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print!("\x1b[1;36mpdf>\x1b[0m ");
        io::stdout().flush()?;

        line.clear();
        let bytes = stdin.read_line(&mut line)?;
        if bytes == 0 {
            println!();
            break;
        }

        match line.trim() {
            "" => continue,
            "help" => print_shell_help(),
            "exit" | "quit" => {
                println!("Bye!");
                break;
            }
            input => {
                if let Err(err) = dispatch_shell_command(input) {
                    println!("error[shell_dispatch]: {err}");
                    println!("Tip: type `help` for shell commands.");
                }
            }
        }
    }
    Ok(())
}

fn dispatch_shell_command(input: &str) -> anyhow::Result<()> {
    let Some(parts) = shlex::split(input) else {
        anyhow::bail!("failed to parse command line");
    };
    if parts.is_empty() {
        return Ok(());
    }

    let mut argv = vec!["pdf".to_string()];
    argv.extend(parts);
    let cli = Cli::try_parse_from(argv)?;
    execute_command(cli)
}

fn print_shell_banner() {
    println!("\x1b[1;35m┌──────────────────────────┐\x1b[0m");
    println!("\x1b[1;35m│\x1b[0m  ✨ PDF Toolkit Shell ✨  \x1b[1;35m│\x1b[0m");
    println!("\x1b[1;35m└──────────────────────────┘\x1b[0m");
}

fn print_shell_help() {
    println!("Shell commands:");
    println!("  help        Show this help");
    println!("  run <pdf-command>  Execute a command explicitly");
    println!("              (plain command input also works)");
    println!("  quit, exit  Leave the interactive shell");
}
