use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

use crate::DashboardMode;

pub async fn run(refresh: u64, mode: DashboardMode) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App state
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_secs(refresh);
    
    // Sample data (in real implementation, fetch from API)
    let mut throughput_data: Vec<u64> = vec![45, 52, 48, 61, 58, 67, 72, 68, 74, 81];
    let mut iteration = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            
            match mode {
                DashboardMode::Compact => render_compact(f, size, &throughput_data),
                DashboardMode::Full => render_full(f, size, &throughput_data),
                DashboardMode::Simple => render_simple(f, size, &throughput_data),
            }
        })?;

        // Handle input
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            
            // Update data (simulate real-time data)
            iteration += 1;
            let new_val = 70 + (iteration % 20) as u64;
            throughput_data.push(new_val);
            if throughput_data.len() > 20 {
                throughput_data.remove(0);
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn render_compact(f: &mut ratatui::Frame, area: Rect, throughput_data: &[u64]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("🚀 ORION Unified CDR Engine - Live Monitor")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)));
    f.render_widget(header, chunks[0]);

    // Services Status
    let services = vec![
        ("API", "✅", "Running", 95),
        ("Ingestion", "✅", "Running", 88),
        ("Validation", "✅", "Running", 92),
        ("Normalization", "✅", "Running", 90),
        ("Enrichment", "✅", "Running", 85),
        ("ML Fraud", "✅", "Running", 78),
        ("Storage Hot", "✅", "Running", 97),
        ("Storage Cold", "✅", "Running", 98),
    ];

    let service_items: Vec<ListItem> = services
        .iter()
        .map(|(name, icon, status, health)| {
            let health_bar = "█".repeat(*health / 10);
            let color = if *health >= 90 {
                Color::Green
            } else if *health >= 70 {
                Color::Yellow
            } else {
                Color::Red
            };
            
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default().fg(Color::Green)),
                Span::styled(format!("{:18}", name), Style::default().fg(Color::White)),
                Span::styled(format!("{:10}", status), Style::default().fg(Color::Cyan)),
                Span::styled(health_bar, Style::default().fg(color)),
            ]))
        })
        .collect();

    let services_list = List::new(service_items)
        .block(
            Block::default()
                .title("🔄 Services")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(services_list, chunks[1]);

    // Throughput Graph
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title(format!("📊 Throughput (req/s) - Current: {} ", throughput_data.last().unwrap_or(&0)))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .data(throughput_data)
        .style(Style::default().fg(Color::Yellow))
        .max(*throughput_data.iter().max().unwrap_or(&100));
    f.render_widget(sparkline, chunks[2]);

    // Stats
    let stats = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("📈 Total CDRs: ", Style::default().fg(Color::Cyan)),
            Span::styled("2,847,392", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("🚨 Fraud: ", Style::default().fg(Color::Cyan)),
            Span::styled("4,721", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled("⚡ Latency: ", Style::default().fg(Color::Cyan)),
            Span::styled("12.4ms", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("💾 Memory: ", Style::default().fg(Color::Cyan)),
            Span::styled("4.2 GB / 16 GB", Style::default().fg(Color::Yellow)),
            Span::raw("  "),
            Span::styled("🖥️  CPU: ", Style::default().fg(Color::Cyan)),
            Span::styled("12.4%", Style::default().fg(Color::Green)),
        ]),
    ])
    .block(
        Block::default()
            .title("📊 Statistics")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(stats, chunks[3]);
}

fn render_full(f: &mut ratatui::Frame, area: Rect, throughput_data: &[u64]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("🚀 ORION Unified CDR Engine - Full Monitor")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)));
    f.render_widget(header, chunks[0]);

    // Services (same as compact)
    render_compact(f, chunks[1], throughput_data);

    // Kafka Lag
    let kafka_items: Vec<ListItem> = vec![
        "cdr-raw: lag 12 ✅",
        "cdr-validated: lag 47 ⚠️",
        "cdr-normalized: lag 1,847 🔴",
        "cdr-enriched: lag 4 ✅",
    ]
    .iter()
    .map(|item| {
        ListItem::new(*item)
            .style(Style::default().fg(Color::White))
    })
    .collect();

    let kafka_list = List::new(kafka_items)
        .block(
            Block::default()
                .title("📊 Kafka Consumer Lag")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(kafka_list, chunks[2]);

    // Infrastructure
    let infra_items: Vec<ListItem> = vec![
        ("ScyllaDB", "✅ Healthy", Color::Green),
        ("Kafka", "✅ Healthy", Color::Green),
        ("MinIO", "✅ Healthy", Color::Green),
        ("Prometheus", "✅ Healthy", Color::Green),
        ("Grafana", "✅ Healthy", Color::Green),
    ]
    .iter()
    .map(|(name, status, color)| {
        ListItem::new(Line::from(vec![
            Span::styled(format!("{:15}", name), Style::default().fg(Color::White)),
            Span::styled(*status, Style::default().fg(*color)),
        ]))
    })
    .collect();

    let infra_list = List::new(infra_items)
        .block(
            Block::default()
                .title("🏗️ Infrastructure")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(infra_list, chunks[3]);
}

fn render_simple(f: &mut ratatui::Frame, area: Rect, _throughput_data: &[u64]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    // Header
    let header = Paragraph::new("🚀 ORION - Simple Monitor")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Cyan)));
    f.render_widget(header, chunks[0]);

    // System Status
    let status = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Cyan)),
            Span::styled("All Systems Operational ✅", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Services: ", Style::default().fg(Color::Cyan)),
            Span::styled("8/8 Running", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("Infrastructure: ", Style::default().fg(Color::Cyan)),
            Span::styled("5/5 Healthy", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total CDRs: ", Style::default().fg(Color::Cyan)),
            Span::styled("2,847,392", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Fraud Detected: ", Style::default().fg(Color::Cyan)),
            Span::styled("4,721", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
    ])
    .block(
        Block::default()
            .title("📊 System Overview")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan)),
    )
    .alignment(Alignment::Center);
    f.render_widget(status, chunks[1]);

    // Controls
    let controls = Paragraph::new("[Q] Quit  [C] Compact  [F] Full")
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("⌨️  Controls")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(controls, chunks[2]);
}
