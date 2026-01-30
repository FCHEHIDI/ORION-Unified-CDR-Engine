mod commands;
mod tui;
mod api;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

/// 🚀 ORION Unified CDR Engine - CLI
#[derive(Parser)]
#[command(
    name = "orion",
    version,
    about = "🚀 ORION CDR Engine CLI - Real-time CDR monitoring and analytics",
    long_about = None,
    styles = get_styles()
)]
struct Cli {
    /// API endpoint URL
    #[arg(long, default_value = "http://localhost:8080")]
    api_url: String,

    /// Output format
    #[arg(short, long, value_enum, default_value = "table")]
    format: OutputFormat,

    /// Disable colors
    #[arg(long)]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, clap::ValueEnum)]
enum OutputFormat {
    Table,
    Json,
    Csv,
    Yaml,
}

#[derive(Subcommand)]
enum Commands {
    /// 🔍 CDR operations (get, search, export)
    Cdr {
        #[command(subcommand)]
        action: CdrCommands,
    },

    /// 📊 Live monitoring dashboard (TUI)
    Monitor {
        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        refresh: u64,

        /// Dashboard mode
        #[arg(short, long, value_enum, default_value = "compact")]
        mode: DashboardMode,
    },

    /// ✅ System health check
    Health {
        /// Show detailed diagnostics
        #[arg(short, long)]
        detailed: bool,
    },

    /// 📋 Service status
    Status {
        /// Service name (empty for all)
        service: Option<String>,
    },

    /// 📝 Service logs
    Logs {
        /// Service name
        service: String,

        /// Follow logs
        #[arg(short, long)]
        follow: bool,

        /// Number of lines
        #[arg(short, long, default_value = "50")]
        tail: usize,
    },

    /// 🚨 Fraud detection operations
    Fraud {
        #[command(subcommand)]
        action: FraudCommands,
    },

    /// 🔧 Kafka operations
    Kafka {
        #[command(subcommand)]
        action: KafkaCommands,
    },

    /// 🗄️ ScyllaDB operations
    Scylla {
        #[command(subcommand)]
        action: ScyllaCommands,
    },
}

#[derive(Subcommand, Clone)]
enum CdrCommands {
    /// Get CDR by ID
    Get {
        /// CDR ID
        id: String,
    },

    /// Search CDRs with filters
    Search {
        /// MSISDN filter
        #[arg(long)]
        msisdn: Option<String>,

        /// IMSI filter
        #[arg(long)]
        imsi: Option<String>,

        /// Call type (voice|sms|data)
        #[arg(long)]
        call_type: Option<String>,

        /// Fraud score minimum (0.0-1.0)
        #[arg(long)]
        fraud_min: Option<f32>,

        /// Only fraudulent CDRs
        #[arg(long)]
        fraud_only: bool,

        /// Time range (e.g., 1h, 24h, 7d)
        #[arg(long)]
        last: Option<String>,

        /// Maximum results
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },

    /// Export CDRs to file
    Export {
        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Export format
        #[arg(long, default_value = "csv")]
        format: String,

        /// Same filters as search
        #[arg(long)]
        msisdn: Option<String>,

        #[arg(long)]
        fraud_only: bool,

        #[arg(long)]
        last: Option<String>,
    },

    /// CDR statistics
    Stats {
        /// Time range
        #[arg(long, default_value = "24h")]
        last: String,

        /// Group by field
        #[arg(long)]
        group_by: Option<String>,
    },
}

#[derive(Subcommand, Clone)]
enum FraudCommands {
    /// Analyze fraud patterns
    Analyze {
        /// Time range
        #[arg(long, default_value = "1h")]
        last: String,

        /// Minimum fraud score
        #[arg(long, default_value = "0.7")]
        threshold: f32,
    },

    /// Interactive fraud dashboard (TUI)
    Dashboard,

    /// ML model information
    Model {
        /// Show model info
        #[arg(long)]
        info: bool,

        /// Show metrics
        #[arg(long)]
        metrics: bool,
    },
}

#[derive(Subcommand, Clone)]
enum KafkaCommands {
    /// Check consumer lag
    Lag {
        /// Topic name
        topic: Option<String>,

        /// All topics
        #[arg(long)]
        all_topics: bool,
    },

    /// List topics
    Topics,
}

#[derive(Subcommand, Clone)]
enum ScyllaCommands {
    /// Cluster status
    Status,

    /// Execute CQL query
    Query {
        /// CQL query
        query: String,
    },
}

#[derive(Clone, clap::ValueEnum)]
enum DashboardMode {
    Compact,
    Full,
    Simple,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .literal(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize colors
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Print banner
    print_banner();

    // Execute command
    match &cli.command {
        Commands::Cdr { action } => commands::cdr::execute(action.clone(), &cli).await?,
        Commands::Monitor { refresh, mode } => tui::monitor::run(*refresh, mode.clone()).await?,
        Commands::Health { detailed } => commands::health::execute(*detailed, &cli).await?,
        Commands::Status { service } => commands::status::execute(service.clone(), &cli).await?,
        Commands::Logs { service, follow, tail } => {
            commands::logs::execute(service, *follow, *tail).await?
        }
        Commands::Fraud { action } => commands::fraud::execute(action.clone(), &cli).await?,
        Commands::Kafka { action } => commands::kafka::execute(action.clone(), &cli).await?,
        Commands::Scylla { action } => commands::scylla::execute(action.clone(), &cli).await?,
    }

    Ok(())
}

fn print_banner() {
    println!("{}", r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   ██████╗ ██████╗ ██╗ ██████╗ ███╗   ██╗                ║
    ║  ██╔═══██╗██╔══██╗██║██╔═══██╗████╗  ██║                ║
    ║  ██║   ██║██████╔╝██║██║   ██║██╔██╗ ██║                ║
    ║  ██║   ██║██╔══██╗██║██║   ██║██║╚██╗██║                ║
    ║  ╚██████╔╝██║  ██║██║╚██████╔╝██║ ╚████║                ║
    ║   ╚═════╝ ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝                ║
    ║                                                           ║
    ║        🚀 Unified CDR Engine - CLI v1.0.0                ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
    "#.bright_cyan());
}
