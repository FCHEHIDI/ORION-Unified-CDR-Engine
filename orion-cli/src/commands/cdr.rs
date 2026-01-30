use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, ContentArrangement, Table};
use crate::{CdrCommands, Cli, api::ApiClient};

pub async fn execute(action: CdrCommands, cli: &Cli) -> Result<()> {
    let client = ApiClient::new(cli.api_url.clone());

    match action {
        CdrCommands::Get { id } => {
            get_cdr(&client, &id).await?;
        }
        CdrCommands::Search {
            msisdn,
            imsi,
            call_type,
            fraud_min,
            fraud_only,
            last: _,
            limit,
        } => {
            search_cdrs(&client, msisdn, imsi, call_type, fraud_min, fraud_only, limit).await?;
        }
        CdrCommands::Export { .. } => {
            println!("{}", "📦 Export feature coming soon...".bright_yellow());
        }
        CdrCommands::Stats { .. } => {
            println!("{}", "📊 Stats feature coming soon...".bright_yellow());
        }
    }

    Ok(())
}

async fn get_cdr(client: &ApiClient, id: &str) -> Result<()> {
    println!("{}", format!("🔍 Fetching CDR: {}", id).bright_cyan().bold());
    
    let cdr = client.get_cdr(id).await?;

    // NÉON style display
    println!("\n{}", "╔══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", format!("║  {} CDR Details                                   [{}]", "📞".bright_yellow(), cdr.cdr_id).bright_magenta());
    println!("{}", "╠══════════════════════════════════════════════════════════════════╣".bright_magenta());
    
    println!("{}", "║                                                                  ║".bright_magenta());
    println!("{}", format!("║  {} Call Information", "📞".bright_cyan()).bright_magenta());
    println!("{}", "║  ─────────────────────────────────────────────────────────────  ║".bright_magenta());
    println!("{}", format!("║  ID             : {}                          ", cdr.cdr_id.bright_green()).bright_magenta());
    println!("{}", format!("║  Type           : {}                                          ", cdr.call_type.bright_yellow()).bright_magenta());
    println!("{}", format!("║  Direction      : {}                                       ", cdr.direction.bright_cyan()).bright_magenta());
    println!("{}", format!("║  Start Time     : {}                        ", cdr.start_time.bright_white()).bright_magenta());
    println!("{}", format!("║  Duration       : {} seconds                           ", cdr.duration.to_string().bright_green()).bright_magenta());
    
    println!("{}", "║                                                                  ║".bright_magenta());
    println!("{}", format!("║  {} Subscriber", "👤".bright_cyan()).bright_magenta());
    println!("{}", "║  ─────────────────────────────────────────────────────────────  ║".bright_magenta());
    println!("{}", format!("║  MSISDN         : {}                                   ", cdr.msisdn.bright_green().bold()).bright_magenta());
    if let Some(imsi) = &cdr.imsi {
        println!("{}", format!("║  IMSI           : {}                                ", imsi.bright_white()).bright_magenta());
    }
    if let Some(imei) = &cdr.imei {
        println!("{}", format!("║  IMEI           : {}                                ", imei.bright_white()).bright_magenta());
    }

    println!("{}", "║                                                                  ║".bright_magenta());
    println!("{}", format!("║  {} Network", "🌍".bright_cyan()).bright_magenta());
    println!("{}", "║  ─────────────────────────────────────────────────────────────  ║".bright_magenta());
    if let Some(cell_id) = &cdr.cell_id {
        println!("{}", format!("║  Cell ID        : {}                                ", cell_id.bright_yellow()).bright_magenta());
    }
    if let Some(location) = &cdr.location {
        if let Some(city) = &location.city {
            println!("{}", format!("║  Location       : {}                   ", city.bright_green()).bright_magenta());
        }
    }

    println!("{}", "║                                                                  ║".bright_magenta());
    println!("{}", format!("║  {} Fraud Detection", "🚨".bright_red()).bright_magenta());
    println!("{}", "║  ─────────────────────────────────────────────────────────────  ║".bright_magenta());
    if let Some(score) = cdr.fraud_score {
        let (risk_level, risk_emoji, _risk_color) = get_risk_level(score);
        println!("{}", format!("║  Score          : {:.2} ({}) {}                             ", 
            score.to_string().bright_white().bold(),
            risk_level,
            risk_emoji).bright_magenta());
    }

    println!("{}", "║                                                                  ║".bright_magenta());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".bright_magenta());
    
    println!("\n{} {} {} {}", 
        "[E] Export".bright_cyan(), 
        "[J] JSON".bright_green(), 
        "[Q] Quit".bright_red(),
        "");

    Ok(())
}

async fn search_cdrs(
    client: &ApiClient,
    msisdn: Option<String>,
    imsi: Option<String>,
    call_type: Option<String>,
    fraud_min: Option<f32>,
    fraud_only: bool,
    limit: usize,
) -> Result<()> {
    println!("{}", "🔍 Searching CDRs...".bright_cyan().bold());

    let filters = crate::api::SearchFilters {
        msisdn,
        imsi,
        call_type,
        fraud_min,
        fraud_only: if fraud_only { Some(true) } else { None },
        limit: Some(limit),
    };

    let cdrs = client.search_cdrs(filters).await?;

    if cdrs.is_empty() {
        println!("\n{}", "⚠️  No CDRs found matching the criteria".bright_yellow());
        return Ok(());
    }

    println!("\n{}", format!("Found {} CDRs", cdrs.len()).bright_green().bold());

    // NÉON table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Time").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Type").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("MSISDN").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Destination").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Fraud").fg(Color::Red).set_alignment(CellAlignment::Center),
            Cell::new("Duration").fg(Color::Cyan).set_alignment(CellAlignment::Center),
        ]);

    for (idx, cdr) in cdrs.iter().enumerate() {
        let (_, risk_emoji, risk_color) = get_risk_level(cdr.fraud_score.unwrap_or(0.0));
        let fraud_display = format!("{:.2} {}", cdr.fraud_score.unwrap_or(0.0), risk_emoji);
        
        let time = cdr.start_time.split('T').nth(1).unwrap_or(&cdr.start_time)[..8].to_string();
        
        table.add_row(vec![
            Cell::new((idx + 1).to_string()).fg(Color::White),
            Cell::new(time).fg(Color::Green),
            Cell::new(&cdr.call_type).fg(Color::Yellow),
            Cell::new(&cdr.msisdn).fg(Color::Green),
            Cell::new(cdr.destination.as_deref().unwrap_or("N/A")).fg(Color::White),
            Cell::new(fraud_display).fg(risk_color),
            Cell::new(format!("{}s", cdr.duration)).fg(Color::Cyan),
        ]);
    }

    println!("{}", table);

    println!("\n{} {} {} {}", 
        "[↑↓] Navigate".bright_cyan(), 
        "[Enter] Details".bright_green(), 
        "[E] Export".bright_yellow(),
        "[Q] Quit".bright_red());

    Ok(())
}

fn get_risk_level(score: f32) -> (colored::ColoredString, &'static str, Color) {
    if score >= 0.9 {
        ("High Risk".bright_red().bold(), "🔴", Color::Red)
    } else if score >= 0.7 {
        ("Medium Risk".bright_yellow().bold(), "🟡", Color::Yellow)
    } else if score >= 0.5 {
        ("Low Risk".bright_green(), "🟢", Color::Green)
    } else {
        ("Safe".bright_cyan(), "", Color::Cyan)
    }
}
