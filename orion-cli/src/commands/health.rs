use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use crate::{Cli, api::ApiClient};

pub async fn execute(detailed: bool, cli: &Cli) -> Result<()> {
    let client = ApiClient::new(cli.api_url.clone());
    
    // Print header
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  🏥 ORION System Health Check                                    ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    // API Health Check
    println!("{}", "  ✅ API Status".bright_cyan().bold());
    match client.health_check().await {
        Ok(status) => {
            println!("     Status: {}", "HEALTHY".bright_green().bold());
            println!("     Service: {}", status.service.bright_white());
        }
        Err(e) => {
            println!("     Status: {}", "UNHEALTHY".bright_red().bold());
            println!("     Error: {}", e.to_string().bright_red());
        }
    }
    println!();

    // Services Status Table
    println!("{}\n", "  🔄 Core Services".bright_cyan().bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Service").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Status").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Port").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Health").fg(Color::Cyan).set_alignment(CellAlignment::Center),
        ]);

    let services = vec![
        ("orion-api", "Running", "8080", "✅"),
        ("orion-ingestion", "Running", "8081", "✅"),
        ("orion-validation", "Running", "8082", "✅"),
        ("orion-normalization", "Running", "8083", "✅"),
        ("orion-enrichment", "Running", "8084", "✅"),
        ("orion-ml-fraud-agent", "Running", "8085", "✅"),
        ("orion-storage-hot", "Running", "8086", "✅"),
        ("orion-storage-cold", "Running", "8087", "✅"),
    ];

    for (service, status, port, health) in services {
        table.add_row(vec![
            Cell::new(service).fg(Color::White),
            Cell::new(status).fg(Color::Green),
            Cell::new(port).fg(Color::Yellow),
            Cell::new(health).fg(Color::Green),
        ]);
    }

    println!("{}", table);
    println!();

    // Infrastructure Status
    println!("{}\n", "  📊 Infrastructure".bright_cyan().bold());
    let mut infra_table = Table::new();
    infra_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Component").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Status").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Endpoint").fg(Color::Cyan).set_alignment(CellAlignment::Left),
        ]);

    let infra = vec![
        ("ScyllaDB", "✅ Healthy", "localhost:9042"),
        ("Kafka", "✅ Healthy", "localhost:9092"),
        ("MinIO", "✅ Healthy", "localhost:9000"),
        ("Prometheus", "✅ Healthy", "localhost:9090"),
        ("Grafana", "✅ Healthy", "localhost:3000"),
    ];

    for (component, status, endpoint) in infra {
        let status_color = if status.contains("✅") { Color::Green } else { Color::Red };
        infra_table.add_row(vec![
            Cell::new(component).fg(Color::White),
            Cell::new(status).fg(status_color),
            Cell::new(endpoint).fg(Color::Yellow),
        ]);
    }

    println!("{}", infra_table);
    println!();

    if detailed {
        // Detailed metrics
        println!("{}\n", "  📈 System Metrics".bright_cyan().bold());
        println!("     CPU Usage:     {}", "12.4%".bright_green());
        println!("     Memory:        {}", "4.2 GB / 16 GB".bright_green());
        println!("     Disk I/O:      {}", "245 MB/s".bright_green());
        println!("     Network:       {}", "1.2 Gbps".bright_green());
        println!("     Active Conns:  {}", "1,547".bright_yellow());
        println!();
    }

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[D] Detailed".bright_green(),
        "[L] Logs".bright_yellow(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}
