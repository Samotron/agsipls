use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod mcp;
mod lsp;
mod ui;

#[derive(Parser)]
#[command(name = "agsipls")]
#[command(about = "AGSi ground model toolkit - CLI, MCP server, and LSP server in one", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate an AGSi file
    Validate {
        /// Path to the AGSi file
        file: PathBuf,
        
        /// Show detailed validation report
        #[arg(short, long)]
        detailed: bool,
    },

    /// Create a new AGSi document or component
    Create {
        #[command(subcommand)]
        item: CreateItem,
    },

    /// Edit an existing AGSi file
    Edit {
        /// Path to the AGSi file
        file: PathBuf,
    },

    /// Open TUI editor
    Tui {
        /// Optional file to edit
        file: Option<PathBuf>,
    },

    /// Extract materials from a ground model
    Extract {
        /// Path to the AGSi file
        file: PathBuf,

        /// Model ID to extract from
        #[arg(short, long)]
        model: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Display information about an AGSi file
    Info {
        /// Path to the AGSi file
        file: PathBuf,

        /// Show materials
        #[arg(long)]
        materials: bool,

        /// Show models
        #[arg(long)]
        models: bool,
    },

    /// Convert AGSi between formats
    Convert {
        /// Input file path
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Output format (json, avro, protobuf)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Interactive form-based creation
    Form {
        /// What to create
        #[arg(value_enum)]
        item: FormItem,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compare two AGSi files
    Diff {
        /// First file to compare
        file1: PathBuf,

        /// Second file to compare
        file2: PathBuf,

        /// Show detailed differences
        #[arg(short, long)]
        detailed: bool,
    },

    /// Show statistics about an AGSi file
    Stats {
        /// Path to the AGSi file
        file: PathBuf,
    },

    /// Run MCP (Model Context Protocol) server
    Mcp {
        /// Enable debug output
        #[arg(short, long)]
        debug: bool,
    },

    /// Run LSP (Language Server Protocol) server
    Lsp {
        /// TCP port to listen on (optional, defaults to stdio)
        #[arg(short, long)]
        port: Option<u16>,
    },
}

#[derive(Subcommand)]
enum CreateItem {
    /// Create a new document
    Document {
        /// Document ID
        #[arg(short, long)]
        id: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Create a new material
    Material {
        /// Material ID
        #[arg(short, long)]
        id: String,

        /// Material name
        #[arg(short, long)]
        name: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Create a new model
    Model {
        /// Model ID
        #[arg(short, long)]
        id: String,

        /// Model name
        #[arg(short, long)]
        name: String,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Clone, clap::ValueEnum)]
enum FormItem {
    Document,
    Material,
    Model,
    Component,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    setup_logging(cli.verbose);

    match cli.command {
        Commands::Validate { file, detailed } => {
            commands::validate::execute(file, detailed).await?;
        }
        Commands::Create { item } => match item {
            CreateItem::Document { id, output } => {
                commands::create::document(id, output).await?;
            }
            CreateItem::Material { id, name, output } => {
                commands::create::material(id, name, output).await?;
            }
            CreateItem::Model { id, name, output } => {
                commands::create::model(id, name, output).await?;
            }
        },
        Commands::Edit { file } => {
            commands::edit::execute(file).await?;
        }
        Commands::Tui { file } => {
            ui::editor::launch_editor(file).await?;
        }
        Commands::Extract {
            file,
            model,
            output,
        } => {
            commands::extract::execute(file, model, output).await?;
        }
        Commands::Info {
            file,
            materials,
            models,
        } => {
            commands::info::execute(file, materials, models).await?;
        }
        Commands::Convert {
            input,
            output,
            format,
        } => {
            commands::convert::execute(input, output, format).await?;
        }
        Commands::Form { item, output } => {
            commands::form::execute(item, output).await?;
        }
        Commands::Diff { file1, file2, detailed } => {
            commands::diff::execute(file1, file2, detailed).await?;
        }
        Commands::Stats { file } => {
            commands::stats::execute(file).await?;
        }
        Commands::Mcp { debug } => {
            if debug {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }
            mcp::run_mcp_server().await?;
        }
        Commands::Lsp { port } => {
            if let Some(port) = port {
                eprintln!("LSP server would listen on port {}", port);
            }
            lsp::run_lsp_server().await?;
        }
    }

    Ok(())
}

fn setup_logging(verbose: bool) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    let filter = if verbose {
        EnvFilter::new("agsi=debug,agsi_core=debug")
    } else {
        EnvFilter::new("agsi=info,agsi_core=info")
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
