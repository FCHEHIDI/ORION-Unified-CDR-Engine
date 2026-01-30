use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use crate::{FraudCommands, Cli};

pub async fn execute(action: FraudCommands, _cli: &Cli) -> Result<()> {
    match action {
        FraudCommands::Analyze { last, threshold } => {
            show_analysis(&last, threshold).await?;
        }
        FraudCommands::Dashboard => {
            show_dashboard().await?;
        }
        FraudCommands::Model { info, metrics } => {
            show_model_info(info, metrics).await?;
        }
    }
    Ok(())
}

async fn show_analysis(timerange: &str, threshold: f32) -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", format!("║  🚨 Fraud Analysis: last {} (threshold: {:.2}){} ║", 
        timerange.bright_white().bold(), 
        threshold,
        " ".repeat(30 - timerange.len())
    ).bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    // Summary Stats
    println!("{}\n", "  📊 Summary".bright_cyan().bold());
    println!("     Total CDRs:        {}", "142,847".bright_white().bold());
    println!("     Fraud Detected:    {} ({})", "247".bright_red().bold(), "0.17%".bright_red());
    println!("     Above Threshold:   {}", "189".bright_yellow().bold());
    println!("     Avg Fraud Score:   {}", format!("{:.2}", 0.34).bright_cyan());
    println!();

    // Top Fraud Patterns
    println!("{}\n", "  🔝 Top Fraud Patterns".bright_cyan().bold());
    let mut patterns_table = Table::new();
    patterns_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("#").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Pattern").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Count").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Avg Score").fg(Color::Cyan).set_alignment(CellAlignment::Right),
        ]);

    patterns_table.add_row(vec![
        Cell::new("1").fg(Color::Yellow),
        Cell::new("Unusual call frequency").fg(Color::White),
        Cell::new("89").fg(Color::Red),
        Cell::new("0.91").fg(Color::Red),
    ]);
    patterns_table.add_row(vec![
        Cell::new("2").fg(Color::Yellow),
        Cell::new("International roaming").fg(Color::White),
        Cell::new("67").fg(Color::Yellow),
        Cell::new("0.78").fg(Color::Yellow),
    ]);
    patterns_table.add_row(vec![
        Cell::new("3").fg(Color::Yellow),
        Cell::new("Multiple IMEI switches").fg(Color::White),
        Cell::new("42").fg(Color::Yellow),
        Cell::new("0.85").fg(Color::Red),
    ]);

    println!("{}", patterns_table);
    println!();

    // Action buttons
    println!("\n{} {} {} {}",
        "[D] Dashboard".bright_cyan(),
        "[M] Model".bright_green(),
        "[E] Export".bright_yellow(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}

async fn show_dashboard() -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  🚨 Fraud Detection Dashboard                                    ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    // Summary Stats
    println!("{}\n", "  📊 Today's Summary".bright_cyan().bold());
    println!("     Total CDRs:        {}", "2,847,392".bright_white().bold());
    println!("     Fraud Detected:    {} ({})", "4,721".bright_red().bold(), "0.17%".bright_red());
    println!("     Blocked:           {}", "3,894".bright_yellow().bold());
    println!("     Under Review:      {}", "827".bright_cyan().bold());
    println!("     False Positives:   {}", "142".bright_green());
    println!();

    // Top Fraud Patterns
    println!("{}\n", "  🔝 Top Fraud Patterns (Last 24h)".bright_cyan().bold());
    let mut patterns_table = Table::new();
    patterns_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("#").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Pattern").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Count").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Trend").fg(Color::Cyan).set_alignment(CellAlignment::Center),
        ]);

    patterns_table.add_row(vec![
        Cell::new("1").fg(Color::Yellow),
        Cell::new("Call frequency anomaly").fg(Color::White),
        Cell::new("1,847").fg(Color::Red),
        Cell::new("📈 +12%").fg(Color::Red),
    ]);
    patterns_table.add_row(vec![
        Cell::new("2").fg(Color::Yellow),
        Cell::new("International roaming fraud").fg(Color::White),
        Cell::new("1,234").fg(Color::Red),
        Cell::new("📉 -5%").fg(Color::Green),
    ]);
    patterns_table.add_row(vec![
        Cell::new("3").fg(Color::Yellow),
        Cell::new("SIM box detection").fg(Color::White),
        Cell::new("892").fg(Color::Yellow),
        Cell::new("➡️ 0%").fg(Color::Cyan),
    ]);

    println!("{}", patterns_table);
    println!();

    // Model Performance
    println!("{}\n", "  🎯 Model Performance".bright_cyan().bold());
    println!("     Accuracy:      {}", "96.8%".bright_green().bold());
    println!("     Precision:     {}", "94.2%".bright_green().bold());
    println!("     Recall:        {}", "91.7%".bright_green().bold());
    println!("     F1 Score:      {}", "92.9%".bright_green().bold());
    println!();

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[A] Analyze".bright_yellow(),
        "[M] Model".bright_green(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}

async fn show_model_info(info: bool, metrics: bool) -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  🤖 ML Fraud Detection Model                                     ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    if info || (!info && !metrics) {
        println!("{}\n", "  📋 Model Information".bright_cyan().bold());
        println!("     Name:           {}", "ORION Fraud Detector v2.1".bright_white().bold());
        println!("     Type:           {}", "Random Forest Classifier".bright_green());
        println!("     Training Date:  {}", "2026-01-15".bright_cyan());
        println!("     Features:       {}", "127".bright_yellow());
        println!("     Dataset Size:   {}", "12.4M records".bright_cyan());
        println!();
    }

    if metrics || (!info && !metrics) {
        println!("{}\n", "  📊 Model Metrics".bright_cyan().bold());
        println!("     Accuracy:       {}", "96.8%".bright_green().bold());
        println!("     Precision:      {}", "94.2%".bright_green().bold());
        println!("     Recall:         {}", "91.7%".bright_green().bold());
        println!("     F1 Score:       {}", "92.9%".bright_green().bold());
        println!("     AUC-ROC:        {}", "0.987".bright_green().bold());
        println!();
    }

    // Action buttons
    println!("\n{} {} {}",
        "[I] Info".bright_cyan(),
        "[M] Metrics".bright_green(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}
