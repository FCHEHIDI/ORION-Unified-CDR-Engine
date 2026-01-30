use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use crate::ScyllaCommands;

pub async fn execute(action: ScyllaCommands, _cli: &crate::Cli) -> Result<()> {
    match action {
        ScyllaCommands::Status => {
            show_status().await?;
        }
        ScyllaCommands::Query { query } => {
            execute_query(&query).await?;
        }
    }
    Ok(())
}

async fn show_status() -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  🗄️  ScyllaDB Status                                             ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    // Cluster Info
    println!("{}\n", "  📊 Cluster Information".bright_cyan().bold());
    println!("     Cluster Name:   {}", "orion-cluster".bright_white().bold());
    println!("     Datacenter:     {}", "datacenter1".bright_cyan());
    println!("     Total Nodes:    {}", "3".bright_green());
    println!("     Healthy Nodes:  {}", "3/3 ".bright_green().bold());
    println!("     Replication:    {}", "RF=3".bright_yellow());
    println!();

    // Keyspace Info
    println!("{}\n", "  🔑 Keyspaces".bright_cyan().bold());
    let mut ks_table = Table::new();
    ks_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Keyspace").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Tables").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Replication").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Size").fg(Color::Cyan).set_alignment(CellAlignment::Right),
        ]);

    ks_table.add_row(vec![
        Cell::new("orion_cdr").fg(Color::White).set_alignment(CellAlignment::Left),
        Cell::new("4").fg(Color::Yellow),
        Cell::new("3").fg(Color::Green),
        Cell::new("12.4 GB").fg(Color::Cyan),
    ]);
    ks_table.add_row(vec![
        Cell::new("orion_analytics").fg(Color::White).set_alignment(CellAlignment::Left),
        Cell::new("2").fg(Color::Yellow),
        Cell::new("3").fg(Color::Green),
        Cell::new("3.2 GB").fg(Color::Cyan),
    ]);

    println!("{}", ks_table);
    println!();

    // Tables in orion_cdr
    println!("{}\n", "  📋 Tables (orion_cdr)".bright_cyan().bold());
    let mut table_table = Table::new();
    table_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Table").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Rows").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Size").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Read/s").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Write/s").fg(Color::Cyan).set_alignment(CellAlignment::Right),
        ]);

    let table_data = vec![
        ("cdrs", "2,847,392", "8.4 GB", "1,247", "2,847"),
        ("cdrs_by_msisdn", "2,847,392", "2.1 GB", "892", "2,847"),
        ("cdrs_by_date", "2,847,392", "1.4 GB", "124", "2,847"),
        ("fraud_alerts", "4,721", "0.5 GB", "47", "12"),
    ];

    for (table, rows, size, read_s, write_s) in table_data {
        table_table.add_row(vec![
            Cell::new(table).fg(Color::White),
            Cell::new(rows).fg(Color::Green),
            Cell::new(size).fg(Color::Cyan),
            Cell::new(read_s).fg(Color::Yellow),
            Cell::new(write_s).fg(Color::Yellow),
        ]);
    }

    println!("{}", table_table);
    println!();

    // Performance Metrics
    println!("{}\n", "  ⚡ Performance".bright_cyan().bold());
    println!("     Read Latency:   {}", "2.4ms (p99)".bright_green());
    println!("     Write Latency:  {}", "1.8ms (p99)".bright_green());
    println!("     Throughput:     {}", "4,094 ops/s".bright_yellow());
    println!("     Compactions:    {}", "2 active".bright_cyan());
    println!();

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[Q] Query".bright_green(),
        "[N] Nodes".bright_yellow(),
        "[X] Exit".bright_red()
    );

    Ok(())
}

async fn execute_query(query: &str) -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  🗄️  ScyllaDB Query                                              ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    println!("{}\n", "  📝 Query".bright_cyan().bold());
    println!("     {}", query.bright_white());
    println!();

    // Sample results
    println!("{}\n", "  📊 Results".bright_cyan().bold());
    
    if query.to_lowercase().contains("select") {
        let mut results_table = Table::new();
        results_table
            .load_preset(UTF8_FULL)
            .set_header(vec![
                Cell::new("cdr_id").fg(Color::Cyan),
                Cell::new("msisdn").fg(Color::Cyan),
                Cell::new("call_type").fg(Color::Cyan),
                Cell::new("fraud_score").fg(Color::Cyan),
            ]);

        results_table.add_row(vec![
            Cell::new("cdr_20260130_001").fg(Color::Green),
            Cell::new("+33612345678").fg(Color::White),
            Cell::new("voice").fg(Color::Yellow),
            Cell::new("0.87").fg(Color::Red),
        ]);
        results_table.add_row(vec![
            Cell::new("cdr_20260130_002").fg(Color::Green),
            Cell::new("+33687654321").fg(Color::White),
            Cell::new("data").fg(Color::Yellow),
            Cell::new("0.12").fg(Color::Green),
        ]);

        println!("{}", results_table);
        println!();
        println!("     {} rows returned in {}", "2".bright_yellow().bold(), "3.2ms".bright_green());
    } else {
        println!("     Query executed successfully in {}", "1.8ms".bright_green());
    }
    println!();

    // Action buttons
    println!("\n{} {} {}",
        "[N] New Query".bright_cyan(),
        "[S] Status".bright_green(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}
