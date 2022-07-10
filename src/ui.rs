use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let rows = app.items.iter().map(|x| {
        let cells = [
            Cell::from(x.count.to_string()),
            // TODO: highlight differently folders and files
            Cell::from(x.direntry.as_ref()),
        ];
        Row::new(cells).height(1)
    });
    let t = Table::new(rows)
        // .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(Style::default().bg(tui::style::Color::Green))
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, chunks[0], &mut app.state);
}