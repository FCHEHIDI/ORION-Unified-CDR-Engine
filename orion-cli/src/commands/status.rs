use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use crate::Cli;

pub async fn execute(service: Option<String>, _cli: &Cli) -> Result<()> {
    
    if let Some(svc) = service {
        // Single service detailed status
        println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
        println!("{}", format!("║   Service: {}{} ║", svc.bright_white().bold(), " ".repeat(53 - svc.len())).bright_magenta());
        println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
        println!();
        
        println!("     Status:       {}", "Running".bright_green().bold());
        println!("     Uptime:       {}", "2d 14h 32m".bright_cyan());
        println!("     Requests/s:   {}", "1,247".bright_yellow());
        println!("     Avg Latency:  {}", "12.4ms".bright_green());
        println!("     Error Rate:   {}", "0.02%".bright_green());
        println!("     Memory:       {}", "512 MB".bright_cyan());
        println!("     CPU:          {}", "8.3%".bright_green());
        println!();
    } else {
        // All services overview
        println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
        println!("{}", "║   All Services Status                                         ║".bright_magenta());
        println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
        println!();

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_header(vec![
                Cell::new("Service").fg(Color::Cyan).set_alignment(CellAlignment::Left),
                Cell::new("Status").fg(Color::Cyan).set_alignment(CellAlignment::Center),
                Cell::new("Uptime").fg(Color::Cyan).set_alignment(CellAlignment::Center),
                Cell::new("Req/s").fg(Color::Cyan).set_alignment(CellAlignment::Right),
                Cell::new("Latency").fg(Color::Cyan).set_alignment(CellAlignment::Right),
                Cell::new("Memory").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            ]);

        let services_data = vec![
            ("orion-api", " Running", "2d 14h", "1,247", "12.4ms", "512 MB"),
            ("orion-ingestion", " Running", "2d 14h", "2,891", "8.2ms", "768 MB"),
            ("orion-validation", " Running", "2d 14h", "2,891", "15.1ms", "384 MB"),
            ("orion-normalization", " Running", "2d 14h", "2,847", "6.8ms", "256 MB"),
            ("orion-enrichment", " Running", "2d 14h", "2,847", "22.3ms", "896 MB"),
            ("orion-ml-fraud-agent", " Running", "2d 14h", "2,847", "45.7ms", "1.2 GB"),
            ("orion-storage-hot", " Running", "2d 14h", "2,802", "3.4ms", "128 MB"),
            ("orion-storage-cold", " Running", "2d 14h", "2,802", "5.1ms", "128 MB"),
            ("orion-traffic-gen", " Running", "2d 14h", "50", "2.1ms", "64 MB"),
        ];

        for (service, status, uptime, req_s, latency, memory) in services_data {
            let status_color = if status.contains("") { Color::Green } else { Color::Red };
            let latency_ms: f32 = latency.trim_end_matches("ms").parse().unwrap_or(0.0);
            let latency_color = if latency_ms < 20.0 { Color::Green } else if latency_ms < 50.0 { Color::Yellow } else { Color::Red };
            
            table.add_row(vec![
                Cell::new(service).fg(Color::White),
                Cell::new(status).fg(status_color),
                Cell::new(uptime).fg(Color::Cyan),
                Cell::new(req_s).fg(Color::Yellow),
                Cell::new(latency).fg(latency_color),
                Cell::new(memory).fg(Color::Cyan),
            ]);
        }

        println!("{}", table);
        println!();

        // Summary
        println!("     Total Throughput:  {}", "17,174 req/s".bright_yellow().bold());
        println!("     Avg Latency:       {}", "13.2ms".bright_green().bold());
        println!("     Total Memory:      {}", "4.3 GB".bright_cyan().bold());
        println!();
    }

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[S] Service".bright_green(),
        "[M] Monitor".bright_yellow(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}
