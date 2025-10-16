use crate::app::{App, AppState, InputField};
use crate::colors::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main layout: Header and Body
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(size);

    // Header
    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            "GRAVYAN",
            Style::default().fg(MAUVE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "LordPaijo (2025) gravyan v0.2.7",
            Style::default().fg(OVERLAY0),
        )),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title("Header")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PEACH))
            .style(Style::default().bg(BASE)),
    );
    f.render_widget(header, chunks[0]);

    // Body with controls at the bottom
    match app.state {
        AppState::Input => draw_input_body(f, chunks[1], app),
        AppState::Result => draw_result_body(f, chunks[1], app),
    }
}

fn draw_input_body(f: &mut Frame, area: Rect, app: &App) {
    // Create body block with controls at bottom
    let body_block = Block::default()
        .title("Input Gravitasi")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .style(Style::default().bg(BASE));

    let inner_area = body_block.inner(area);
    f.render_widget(body_block, area);

    // Split inner area for content and controls
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner_area);

    // Content area
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(5)])
        .split(body_chunks[0]);

    // Title and formula
    let title = Paragraph::new(vec![
        Line::from(Span::styled(
            "Hitung Gravitasi dengan Gerak Lurus Berubah Beraturan",
            Style::default().fg(TEXT),
        )),
        Line::from(Span::styled(
            "v² = 2gh  →  g = v² / 2h",
            Style::default().fg(TEXT),
        )),
    ])
    .alignment(Alignment::Center)
    .style(Style::default().bg(BASE));
    f.render_widget(title, content_chunks[0]);

    // Input fields
    let input_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunks[1]);

    let height_selected = matches!(app.selected_field, InputField::Height);
    let height_block = Block::default()
        .title("Height (m):")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if height_selected { BLUE } else { PEACH }))
        .style(Style::default().bg(if height_selected { MANTLE } else { BASE }));
    let height_text = Paragraph::new(app.height_input.as_str())
        .block(height_block)
        .style(Style::default().fg(TEXT));
    f.render_widget(height_text, input_chunks[0]);

    let velocity_selected = matches!(app.selected_field, InputField::Velocity);
    let velocity_block = Block::default()
        .title("Velocity (m/s):")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if velocity_selected { BLUE } else { PEACH }))
        .style(Style::default().bg(if velocity_selected { MANTLE } else { BASE }));
    let velocity_text = Paragraph::new(app.velocity_input.as_str())
        .block(velocity_block)
        .style(Style::default().fg(TEXT));
    f.render_widget(velocity_text, input_chunks[1]);

    // Controls at bottom
    let controls = Line::from(vec![
        Span::styled("[Esc] ", Style::default().fg(BLUE)),
        Span::styled("Menu ", Style::default().fg(TEXT)),
        Span::styled("[←/→] ", Style::default().fg(BLUE)),
        Span::styled("Kanan/Kiri ", Style::default().fg(TEXT)),
        Span::styled("[Enter] ", Style::default().fg(BLUE)),
        Span::styled("Hitung ", Style::default().fg(TEXT)),
        Span::styled("[Q] ", Style::default().fg(RED)),
        Span::styled("Keluar", Style::default().fg(TEXT)),
    ]);
    let controls_widget = Paragraph::new(controls)
        .alignment(Alignment::Center)
        .style(Style::default().bg(BASE));
    f.render_widget(controls_widget, body_chunks[1]);
}

fn draw_result_body(f: &mut Frame, area: Rect, app: &App) {
    // Create body block with controls at bottom
    let body_block = Block::default()
        .title("Hasil Perhitungan")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(GREEN))
        .style(Style::default().bg(BASE));

    let inner_area = body_block.inner(area);
    f.render_widget(body_block, area);

    // Split inner area for content and controls
    let body_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner_area);

    if let Some(result) = &app.result {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("Height = {:.2} m", result.height),
                Style::default().fg(TEXT),
            ))
            .alignment(Alignment::Center),
            Line::from(Span::styled(
                format!("Velocity = {:.2} m/s", result.velocity),
                Style::default().fg(TEXT),
            ))
            .alignment(Alignment::Center),
            Line::from("").alignment(Alignment::Center),
            Line::from(Span::styled(
                "Formula:",
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
            Line::from(Span::styled("g = v² / (2 × h)", Style::default().fg(TEXT)))
                .alignment(Alignment::Center),
            Line::from("").alignment(Alignment::Center),
            Line::from(Span::styled(
                format!("g = ({:.2}²) / (2 × {:.2})", result.velocity, result.height),
                Style::default().fg(TEXT),
            ))
            .alignment(Alignment::Center),
            Line::from(Span::styled(
                format!("g = {:.4} m/s²", result.gravity),
                Style::default().fg(MAUVE).add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
        ];

        let result_widget = Paragraph::new(content).style(Style::default().bg(BASE));
        f.render_widget(result_widget, body_chunks[0]);
    }

    // Controls at bottom
    let controls = Line::from(vec![
        Span::styled("[Esc] ", Style::default().fg(BLUE)),
        Span::styled("Menu ", Style::default().fg(TEXT)),
        Span::styled("[Q] ", Style::default().fg(RED)),
        Span::styled("Keluar", Style::default().fg(TEXT)),
    ]);
    let controls_widget = Paragraph::new(controls)
        .alignment(Alignment::Center)
        .style(Style::default().bg(BASE));
    f.render_widget(controls_widget, body_chunks[1]);
}
