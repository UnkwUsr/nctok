use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
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

    let rows = app
        .current()
        .children
        .as_ref()
        .map(|y| {
            y.iter()
                .map(|x| {
                    let cells = [
                        Cell::from(x.1.size.to_string()),
                        // colorizing if entry have children
                        {
                            let t = Cell::from(x.0.to_string());
                            if x.1.children.is_some() {
                                t.style(
                                    Style::default()
                                        .fg(tui::style::Color::LightBlue)
                                        .add_modifier(Modifier::BOLD),
                                )
                            } else {
                                t
                            }
                        },
                    ];
                    Row::new(cells).height(1)
                })
                .collect()
        })
        .unwrap_or(Vec::new());
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
