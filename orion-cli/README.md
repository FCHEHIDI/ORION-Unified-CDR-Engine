# 🚀 ORION CLI - Unified CDR Engine Command-Line Interface

> **NEON-styled terminal interface for the ORION CDR Engine with full TUI support**

## 🌟 Features

- **🔍 CDR Operations** - Search, view, and export CDR records with NEON colored tables
- **📊 Live TUI Dashboard** - Real-time monitoring with ratatui (3 modes: Compact, Full, Simple)
- **✅ System Health** - Beautiful health checks with colored status indicators
- **📋 Service Status** - Monitor all microservices with performance metrics
- **📝 Colored Logs** - Tail service logs with level-based color coding
- **🚨 Fraud Detection** - Analyze fraud patterns and view ML model metrics
- **🔧 Kafka Monitoring** - Consumer lag tracking and topic management
- **🗄️ ScyllaDB Tools** - Database status and query execution

## 🎨 NEON Styling

The CLI features a distinctive **NEON terminal aesthetic** with:
- Bright magenta UTF-8 borders (`╔═══╗`)
- Color-coded emojis for visual clarity
- Risk-level indicators: 🔴 High | 🟡 Medium | 🟢 Low | ✅ Safe
- Cyan/Green/Yellow/Red color scheme matching RHEL Linux terminals

## 📦 Installation

### Prerequisites
- Rust 1.70+ (for building from source)
- Access to ORION API endpoint (default: `http://localhost:8080`)

### Build from Source
```bash
cd orion-cli
cargo build --release
```

The binary will be available at `target/release/orion` (or `orion.exe` on Windows).

### Add to PATH (Optional)
```bash
# Linux/macOS
sudo cp target/release/orion /usr/local/bin/

# Windows
# Add target/release to your PATH environment variable
```

## 🚀 Quick Start

### View System Health
```bash
orion health
orion health --detailed  # Show detailed metrics
```

### Search CDRs
```bash
# Search by MSISDN
orion cdr search --msisdn +33612345678

# Search fraud only
orion cdr search --fraud-only --limit 50

# Search by call type
orion cdr search --call-type voice --last 1h
```

### Get Single CDR
```bash
orion cdr get cdr_20260130_001
```

### Live Monitor Dashboard
```bash
# Compact mode (default)
orion monitor

# Full dashboard
orion monitor --mode full

# Simple view
orion monitor --mode simple --refresh 5
```

### Service Status
```bash
# All services
orion status

# Specific service
orion status --service orion-api
```

### Fraud Analysis
```bash
# Analyze fraud patterns
orion fraud analyze --threshold 0.7

# Show fraud dashboard
orion fraud dashboard

# ML model metrics
orion fraud model --metrics
```

### Kafka Operations
```bash
# Check consumer lag
orion kafka lag

# List topics
orion kafka topics
```

### ScyllaDB Operations
```bash
# Database status
orion scylla status

# Execute query
orion scylla query "SELECT * FROM orion_cdr.cdrs LIMIT 10"
```

### View Logs
```bash
# Tail logs
orion logs orion-api

# Follow mode
orion logs orion-ingestion --follow --tail 100
```

## 🎯 Command Reference

### Global Flags
- `--api-url <URL>` - API endpoint (default: `http://localhost:8080`)
- `-f, --format <FORMAT>` - Output format: `table`, `json`, `csv`, `yaml`
- `--no-color` - Disable colored output

### Commands

#### `cdr` - CDR Operations
```bash
orion cdr get <ID>                    # Get CDR by ID
orion cdr search [OPTIONS]            # Search CDRs
orion cdr export [OPTIONS]            # Export to file
orion cdr stats [OPTIONS]             # Statistics
```

**Search Options:**
- `--msisdn <MSISDN>` - Filter by phone number
- `--imsi <IMSI>` - Filter by IMSI
- `--call-type <TYPE>` - Filter by type (voice, sms, data)
- `--fraud-min <SCORE>` - Minimum fraud score (0.0-1.0)
- `--fraud-only` - Show only fraudulent CDRs
- `--last <RANGE>` - Time range (e.g., 1h, 24h, 7d)
- `-l, --limit <N>` - Maximum results (default: 100)

#### `monitor` - Live TUI Dashboard
```bash
orion monitor                         # Launch compact dashboard
orion monitor --mode full             # Full dashboard
orion monitor --mode simple           # Simple view
orion monitor --refresh <SECS>        # Refresh interval (default: 2)
```

**Dashboard Modes:**
- `compact` - 8 services, throughput graph, quick stats
- `full` - Services, Kafka lag, infrastructure, detailed metrics
- `simple` - System overview with minimal UI

#### `health` - System Health
```bash
orion health                          # Quick health check
orion health --detailed               # Detailed diagnostics
```

#### `status` - Service Status
```bash
orion status                          # All services
orion status --service <NAME>         # Specific service
```

#### `logs` - Service Logs
```bash
orion logs <SERVICE>                  # Tail 50 lines
orion logs <SERVICE> --follow         # Follow mode
orion logs <SERVICE> --tail <N>       # Show N lines
```

#### `fraud` - Fraud Detection
```bash
orion fraud analyze [OPTIONS]         # Analyze patterns
orion fraud dashboard                 # TUI dashboard
orion fraud model [OPTIONS]           # Model info/metrics
```

#### `kafka` - Kafka Operations
```bash
orion kafka lag [--topic <TOPIC>]     # Consumer lag
orion kafka topics                    # List topics
```

#### `scylla` - ScyllaDB Operations
```bash
orion scylla status                   # Database status
orion scylla query <QUERY>            # Execute CQL query
```

## 🎨 Output Examples

### CDR Search (NEON Table)
```
┌───┬──────────┬───────┬───────────────┬────────────┬──────────┬──────────┐
│ # ┆   Time   ┆ Type  ┆    MSISDN     ┆ Destination┆  Fraud   ┆ Duration │
╞═══╪══════════╪═══════╪═══════════════╪════════════╪══════════╪══════════╡
│ 1 ┆ 14:32:18 ┆ voice ┆ +33612345678  ┆ +33987...  ┆ 0.87 🔴  ┆ 247s     │
│ 2 ┆ 14:33:42 ┆ data  ┆ +33687654321  ┆ N/A        ┆ 0.12 ✅  ┆ 0s       │
└───┴──────────┴───────┴───────────────┴────────────┴──────────┴──────────┘
```

### Live Monitor (Compact Mode)
```
╔═══════════════════════════════════════════════════════════╗
║  🚀 ORION Unified CDR Engine - Live Monitor              ║
╚═══════════════════════════════════════════════════════════╝

  🔄 Core Services
  ✅ orion-api           Running    ██████████ 95%
  ✅ orion-ingestion     Running    ████████░░ 88%
  ...

  📊 Throughput (req/s) - Current: 81
  ▁▂▃▄▅▆▇█▇▆▅▄▃▂▁

  📈 Total CDRs: 2,847,392   🚨 Fraud: 4,721   ⚡ Latency: 12.4ms
```

## 🛠️ Development

### Project Structure
```
orion-cli/
├── src/
│   ├── main.rs              # Entry point + command routing
│   ├── api/
│   │   ├── mod.rs
│   │   └── client.rs        # REST API client
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── cdr.rs           # CDR commands (NEON tables)
│   │   ├── health.rs        # Health check
│   │   ├── status.rs        # Service status
│   │   ├── logs.rs          # Log streaming
│   │   ├── fraud.rs         # Fraud analysis
│   │   ├── kafka.rs         # Kafka monitoring
│   │   └── scylla.rs        # ScyllaDB tools
│   └── tui/
│       ├── mod.rs
│       └── monitor.rs       # Ratatui dashboards
├── Cargo.toml
└── README.md
```

### Dependencies
- **CLI**: `clap` 4.5 (parsing, colored help)
- **TUI**: `ratatui` 0.26, `crossterm` 0.27
- **HTTP**: `reqwest` 0.11, `tokio` 1.35
- **Colors**: `colored` 2.1, `owo-colors` 4.0
- **Tables**: `comfy-table` 7.1, `tabled` 0.15
- **Time**: `chrono` 0.4
- **Error**: `anyhow` 1.0

### Build Modes
```bash
# Debug build (fast compile, slower runtime)
cargo build

# Release build (optimized)
cargo build --release

# Check compilation without building
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Adding New Commands
1. Create module in `src/commands/<name>.rs`
2. Add enum variant to `Commands` in `main.rs`
3. Implement `execute()` function with NEON styling
4. Use `comfy_table::Color` for table cells
5. Use `colored::Colorize` for text colors

## 📝 Configuration

The CLI uses environment variables and command-line flags:

```bash
# Set default API URL
export ORION_API_URL="http://production-api:8080"

# Use in commands
orion cdr search --msisdn +33612345678

# Override per-command
orion --api-url http://localhost:8080 health
```

## 🎯 Keyboard Shortcuts

### TUI Dashboard (Monitor)
- `q` or `Esc` - Quit dashboard
- Auto-refresh every 2 seconds (configurable)

### Navigation Hints
Commands display action buttons at the bottom:
- `[R] Refresh` - Refresh data
- `[E] Export` - Export results
- `[Q] Quit` - Exit command
- `[↑↓] Navigate` - Scroll through results

## 🐛 Troubleshooting

### API Connection Errors
```bash
# Check API is running
curl http://localhost:8080/health

# Use different endpoint
orion --api-url http://localhost:8080 health
```

### Colors Not Showing
```bash
# Check terminal supports colors
echo $TERM  # Should be xterm-256color or similar

# Force enable colors
# (Colors auto-detect terminal capabilities)

# Disable colors if needed
orion --no-color health
```

### Build Errors
```bash
# Update Rust toolchain
rustup update stable

# Clean and rebuild
cargo clean
cargo build --release
```

## 📊 Performance

- **Binary Size**: ~3.5 MB (release build with optimizations)
- **Startup Time**: <50ms
- **Memory Usage**: ~5-10 MB (CLI), ~15-20 MB (TUI dashboard)
- **API Latency**: Depends on network/orion-api response time

## 🔐 Security Notes

- CLI does not store credentials
- All API calls use HTTP (add HTTPS in production)
- No authentication implemented yet (TODO)
- Logs may contain sensitive CDR data

## 🗺️ Roadmap

- [ ] Add JWT authentication for API
- [ ] Implement CDR export (CSV/JSON)
- [ ] Add CDR statistics command
- [ ] Interactive fraud analysis (drill-down)
- [ ] Config file support (`~/.orion/config.toml`)
- [ ] Shell completion (bash, zsh, fish)
- [ ] Docker image for CLI
- [ ] CI/CD integration examples
- [ ] Man pages

## 📄 License

Part of the ORION Unified CDR Engine project.

## 👥 Contributing

See main ORION project documentation.

## 📞 Support

For issues, questions, or contributions, see the main ORION repository.

---

**Built with ❤️ using Rust and NEON aesthetics**
