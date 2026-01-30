use anyhow::Result;
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, CellAlignment, Color, Table};
use crate::KafkaCommands;

pub async fn execute(action: KafkaCommands, _cli: &crate::Cli) -> Result<()> {
    match action {
        KafkaCommands::Lag { topic, all_topics } => {
            show_consumer_lag(topic, all_topics).await?;
        }
        KafkaCommands::Topics => {
            show_topics().await?;
        }
    }
    Ok(())
}

async fn show_consumer_lag(topic: Option<String>, all_topics: bool) -> Result<()> {
    let title = if all_topics {
        "All Topics".to_string()
    } else if let Some(t) = topic {
        t
    } else {
        "All Groups".to_string()
    };
    
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", format!("║  📊 Kafka Consumer Lag: {}{} ║", 
        title.bright_white().bold(), 
        " ".repeat(45 - title.len())
    ).bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    let mut lag_table = Table::new();
    lag_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Group").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Topic").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Partition").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Current").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Lag").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Status").fg(Color::Cyan).set_alignment(CellAlignment::Center),
        ]);

    let lag_data = vec![
        ("orion-ingestion", "cdr-raw", "0", "1,247,392", "12", "✅"),
        ("orion-ingestion", "cdr-raw", "1", "1,251,847", "8", "✅"),
        ("orion-validation", "cdr-validated", "0", "1,247,380", "47", "⚠️"),
        ("orion-validation", "cdr-validated", "1", "1,251,839", "124", "⚠️"),
        ("orion-enrichment", "cdr-normalized", "0", "1,247,333", "1,847", "🔴"),
        ("orion-enrichment", "cdr-normalized", "1", "1,251,715", "2,234", "🔴"),
        ("orion-fraud", "cdr-enriched", "0", "1,245,486", "4", "✅"),
        ("orion-fraud", "cdr-enriched", "1", "1,249,481", "7", "✅"),
    ];

    for (group, topic_name, partition, current, lag, status) in lag_data {
        let lag_num: i32 = lag.replace(",", "").parse().unwrap_or(0);
        let (lag_color, status_color) = if lag_num > 1000 {
            (Color::Red, Color::Red)
        } else if lag_num > 100 {
            (Color::Yellow, Color::Yellow)
        } else {
            (Color::Green, Color::Green)
        };

        lag_table.add_row(vec![
            Cell::new(group).fg(Color::White),
            Cell::new(topic_name).fg(Color::Cyan),
            Cell::new(partition).fg(Color::Yellow),
            Cell::new(current).fg(Color::White),
            Cell::new(lag).fg(lag_color),
            Cell::new(status).fg(status_color),
        ]);
    }

    println!("{}", lag_table);
    println!();

    // Summary
    println!("{}\n", "  📈 Summary".bright_cyan().bold());
    println!("     Total Lag:      {}", "4,283".bright_yellow().bold());
    println!("     Healthy:        {} consumers", "4".bright_green());
    println!("     Warning:        {} consumers", "2".bright_yellow());
    println!("     Critical:       {} consumers", "2".bright_red());
    println!();

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[G] Group".bright_green(),
        "[T] Topics".bright_yellow(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}

async fn show_topics() -> Result<()> {
    println!("{}", "╔═══════════════════════════════════════════════════════════════════╗".bright_magenta());
    println!("{}", "║  📊 Kafka Topics                                                 ║".bright_magenta());
    println!("{}", "╚═══════════════════════════════════════════════════════════════════╝".bright_magenta());
    println!();

    let mut topics_table = Table::new();
    topics_table
        .load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Topic").fg(Color::Cyan).set_alignment(CellAlignment::Left),
            Cell::new("Partitions").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Replication").fg(Color::Cyan).set_alignment(CellAlignment::Center),
            Cell::new("Messages").fg(Color::Cyan).set_alignment(CellAlignment::Right),
            Cell::new("Size").fg(Color::Cyan).set_alignment(CellAlignment::Right),
        ]);

    let topics_data = vec![
        ("cdr-raw", "2", "1", "2,499,239", "487 MB"),
        ("cdr-validated", "2", "1", "2,499,219", "482 MB"),
        ("cdr-normalized", "2", "1", "2,499,048", "478 MB"),
        ("cdr-enriched", "2", "1", "2,494,967", "521 MB"),
        ("cdr-fraud-detected", "2", "1", "4,721", "892 KB"),
    ];

    for (topic, partitions, replication, messages, size) in topics_data {
        topics_table.add_row(vec![
            Cell::new(topic).fg(Color::White),
            Cell::new(partitions).fg(Color::Yellow),
            Cell::new(replication).fg(Color::Cyan),
            Cell::new(messages).fg(Color::Green),
            Cell::new(size).fg(Color::Cyan),
        ]);
    }

    println!("{}", topics_table);
    println!();

    // Summary
    println!("     Total Topics:   {}", "5".bright_yellow().bold());
    println!("     Total Messages: {}", "9,997,194".bright_green().bold());
    println!("     Total Size:     {}", "1.94 GB".bright_cyan().bold());
    println!();

    // Action buttons
    println!("\n{} {} {} {}",
        "[R] Refresh".bright_cyan(),
        "[L] Lag".bright_green(),
        "[D] Describe".bright_yellow(),
        "[Q] Quit".bright_red()
    );

    Ok(())
}
